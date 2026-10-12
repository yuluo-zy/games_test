// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tg-mods/src/discovery.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 9786c9ed9b2f2f339fe24ceffaf84be244625be49c59988c51fc02feff6b30ca
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: discovery.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 683, 2029, 2045, 2046, 2052, 3559
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.6"}]
// use blake3; // namespace candidate
//   package identities: [{"name":"blake3","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.3"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use zip; // namespace candidate
//   package identities: [{"name":"zip","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::mods_root
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1CA
// 首条位置: {"module":683,"inlinee":45514}
// 编译实例 RVAs: 0xd96190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1700753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382955408,"raw_hex":"e891f31900","size":5,"soft_fail":false,"target_va":5384656240,"va":5382955482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t184111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382955408,"raw_hex":"e82fcf0200","size":5,"soft_fail":false,"target_va":5383139568,"va":5382955452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var_os
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1756865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382955408,"raw_hex":"e8c1ce1a00","size":5,"soft_fail":false,"target_va":5384712304,"va":5382955434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":2156,"parent_record_offset":2092,"ancestor_record_offset":2092,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":2192,"parent_record_offset":2092,"ancestor_record_offset":2092,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::env::var_os
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\env.rs
//     evidence={"module":683,"record_offset":2128,"parent_record_offset":2092,"ancestor_record_offset":2092,"inlinee":10543,"source_line":259,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":2092,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mods_root::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2224,"parent_record_offset":2192,"ancestor_record_offset":2092,"inlinee":45516,"source_line":16,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::mods_root::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1CC
// 首条位置: {"module":683,"inlinee":45516}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::mod_cache_root
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1D7
// 首条位置: {"module":683,"inlinee":45527}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":12752,"parent_record_offset":12728,"ancestor_record_offset":12728,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::mod_parts
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":62628,"rva":14240336,"va":5382949456}
// 编译实例 RVAs: 0xd94a50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1737713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8f1831a00","size":5,"soft_fail":false,"target_va":5384689584,"va":5382951866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1816565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8f5b71b00","size":5,"soft_fail":false,"target_va":5384768304,"va":5382951734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1695357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e87dde1900","size":5,"soft_fail":false,"target_va":5384644912,"va":5382949550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1706402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8a2091a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5382949833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1810695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e807a11b00","size":5,"soft_fail":false,"target_va":5384764464,"va":5382953764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <str>::to_lowercase
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1836759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8d7061c00","size":5,"soft_fail":false,"target_va":5384787088,"va":5382950324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1658794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8aa4f1900","size":5,"soft_fail":false,"target_va":5384609408,"va":5382950609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7273343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8810491ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5382952026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=18
//     evidence={"assembly":"\tcallq\t-7271941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8fb0991ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382950640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14229819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8c5de26ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5382952006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13619963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8052d30ff","size":5,"soft_fail":false,"target_va":5369335008,"va":5382954966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13610943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8415030ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5382949786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13523876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e85ca431ff","size":5,"soft_fail":false,"target_va":5369431360,"va":5382955231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bbcc0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13476153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8c75e32ff","size":5,"soft_fail":false,"target_va":5369478336,"va":5382954484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040fae0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9986063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8f19f67ff","size":5,"soft_fail":false,"target_va":5372967648,"va":5382953706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9468989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8c3836fff","size":5,"soft_fail":false,"target_va":5373480896,"va":5382949880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4222120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e85893bfff","size":5,"soft_fail":false,"target_va":5378733248,"va":5382955363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33397344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8609afd01","size":5,"soft_fail":false,"target_va":5416350944,"va":5382953595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34105982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e87e6a0802","size":5,"soft_fail":false,"target_va":5417057040,"va":5382951053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34107298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8a26f0802","size":5,"soft_fail":false,"target_va":5417057056,"va":5382949753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33666946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e882b70102","size":5,"soft_fail":false,"target_va":5416620794,"va":5382953843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-829167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e81159f3ff","size":5,"soft_fail":false,"target_va":5382122800,"va":5382951962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::ReadDir,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-28708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8dc8fffff","size":5,"soft_fail":false,"target_va":5382926368,"va":5382955071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1878179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8a3a81c00","size":5,"soft_fail":false,"target_va":5384830080,"va":5382951896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1775463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e867171b00","size":5,"soft_fail":false,"target_va":5384725152,"va":5382949684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1772829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e81d0d1b00","size":5,"soft_fail":false,"target_va":5384726688,"va":5382953854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::discovery::has_exact_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382949456,"raw_hex":"e8b2edffff","size":5,"soft_fail":false,"target_va":5382945424,"va":5382950105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$93::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":683,"record_offset":64968,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":21307,"source_line":2666,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":63536,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":73784,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":64764,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":683,"record_offset":64820,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":683,"record_offset":68724,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":6131,"source_line":36,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":65208,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":683,"record_offset":70628,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":71064,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":63632,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":70952,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":36640,"source_line":619,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":683,"record_offset":72696,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":67448,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":65264,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":65124,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":63688,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":74548,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":62828,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":5639,"source_line":149,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":70980,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":70864,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":13254,"source_line":2610,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":63564,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":63660,"parent_record_offset":62628,"ancestor_record_offset":62628,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":71584,"parent_record_offset":71560,"ancestor_record_offset":62628,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":71608,"parent_record_offset":71584,"ancestor_record_offset":62628,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":65552,"parent_record_offset":65528,"ancestor_record_offset":62628,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":65576,"parent_record_offset":65552,"ancestor_record_offset":62628,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":71632,"parent_record_offset":71608,"ancestor_record_offset":62628,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":65600,"parent_record_offset":65576,"ancestor_record_offset":62628,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":71560,"parent_record_offset":71404,"ancestor_record_offset":62628,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":65528,"parent_record_offset":65472,"ancestor_record_offset":62628,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":73888,"parent_record_offset":73856,"ancestor_record_offset":62628,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":71336,"parent_record_offset":71304,"ancestor_record_offset":62628,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":65500,"parent_record_offset":65472,"ancestor_record_offset":62628,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":65472,"parent_record_offset":65448,"ancestor_record_offset":62628,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":73936,"parent_record_offset":73912,"ancestor_record_offset":62628,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":73912,"parent_record_offset":73888,"ancestor_record_offset":62628,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":71404,"parent_record_offset":71368,"ancestor_record_offset":62628,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":71368,"parent_record_offset":71336,"ancestor_record_offset":62628,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":65448,"parent_record_offset":65424,"ancestor_record_offset":62628,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":71448,"parent_record_offset":71404,"ancestor_record_offset":62628,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":71200,"parent_record_offset":71168,"ancestor_record_offset":62628,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":71232,"parent_record_offset":71200,"ancestor_record_offset":62628,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":71264,"parent_record_offset":71232,"ancestor_record_offset":62628,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":71168,"parent_record_offset":71136,"ancestor_record_offset":62628,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":683,"record_offset":71136,"parent_record_offset":71104,"ancestor_record_offset":62628,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":72220,"parent_record_offset":72184,"ancestor_record_offset":62628,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":72184,"parent_record_offset":72160,"ancestor_record_offset":62628,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":73856,"parent_record_offset":73812,"ancestor_record_offset":62628,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":73812,"parent_record_offset":73784,"ancestor_record_offset":62628,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":71304,"parent_record_offset":71264,"ancestor_record_offset":62628,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":683,"record_offset":66760,"parent_record_offset":66736,"ancestor_record_offset":62628,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":683,"record_offset":67148,"parent_record_offset":67124,"ancestor_record_offset":62628,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":683,"record_offset":71472,"parent_record_offset":71448,"ancestor_record_offset":62628,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":683,"record_offset":63388,"parent_record_offset":63364,"ancestor_record_offset":62628,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":683,"record_offset":63292,"parent_record_offset":63088,"ancestor_record_offset":62628,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":64420,"parent_record_offset":64392,"ancestor_record_offset":62628,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":683,"record_offset":65968,"parent_record_offset":65944,"ancestor_record_offset":62628,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":69448,"parent_record_offset":69424,"ancestor_record_offset":62628,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":63980,"parent_record_offset":63956,"ancestor_record_offset":62628,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":64064,"parent_record_offset":64040,"ancestor_record_offset":62628,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":64588,"parent_record_offset":63844,"ancestor_record_offset":62628,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":71520,"parent_record_offset":71496,"ancestor_record_offset":62628,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":70652,"parent_record_offset":70628,"ancestor_record_offset":62628,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":70704,"parent_record_offset":70676,"ancestor_record_offset":62628,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":70676,"parent_record_offset":70652,"ancestor_record_offset":62628,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":683,"record_offset":63256,"parent_record_offset":63232,"ancestor_record_offset":62628,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":70376,"parent_record_offset":70348,"ancestor_record_offset":62628,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":70404,"parent_record_offset":70376,"ancestor_record_offset":62628,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":67276,"parent_record_offset":67248,"ancestor_record_offset":62628,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":683,"record_offset":73260,"parent_record_offset":73220,"ancestor_record_offset":62628,"inlinee":29090,"source_line":694,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_uppercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":683,"record_offset":73588,"parent_record_offset":73424,"ancestor_record_offset":62628,"inlinee":18736,"source_line":816,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":683,"record_offset":73424,"parent_record_offset":73260,"ancestor_record_offset":62628,"inlinee":18735,"source_line":668,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":63416,"parent_record_offset":63364,"ancestor_record_offset":62628,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":683,"record_offset":64212,"parent_record_offset":64184,"ancestor_record_offset":62628,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":683,"record_offset":64184,"parent_record_offset":64156,"ancestor_record_offset":62628,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":70752,"parent_record_offset":70728,"ancestor_record_offset":62628,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":63000,"parent_record_offset":62968,"ancestor_record_offset":62628,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":70728,"parent_record_offset":70704,"ancestor_record_offset":62628,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":65232,"parent_record_offset":65208,"ancestor_record_offset":62628,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":63120,"parent_record_offset":63088,"ancestor_record_offset":62628,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":683,"record_offset":69480,"parent_record_offset":69396,"ancestor_record_offset":62628,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":683,"record_offset":71688,"parent_record_offset":71264,"ancestor_record_offset":62628,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":64004,"parent_record_offset":63980,"ancestor_record_offset":62628,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=77
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":65400,"parent_record_offset":65376,"ancestor_record_offset":62628,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":64788,"parent_record_offset":64764,"ancestor_record_offset":62628,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":69936,"parent_record_offset":69912,"ancestor_record_offset":62628,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":63208,"parent_record_offset":63088,"ancestor_record_offset":62628,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":64328,"parent_record_offset":64304,"ancestor_record_offset":62628,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":64880,"parent_record_offset":64820,"ancestor_record_offset":62628,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":72252,"parent_record_offset":72220,"ancestor_record_offset":62628,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":64908,"parent_record_offset":64820,"ancestor_record_offset":62628,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":64852,"parent_record_offset":64820,"ancestor_record_offset":62628,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":64932,"parent_record_offset":64908,"ancestor_record_offset":62628,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":63232,"parent_record_offset":63208,"ancestor_record_offset":62628,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":73976,"parent_record_offset":73812,"ancestor_record_offset":62628,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":63592,"parent_record_offset":63564,"ancestor_record_offset":62628,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":70776,"parent_record_offset":70752,"ancestor_record_offset":62628,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":683,"record_offset":73168,"parent_record_offset":73136,"ancestor_record_offset":62628,"inlinee":29088,"source_line":60,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case_simple
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":683,"record_offset":73220,"parent_record_offset":73168,"ancestor_record_offset":62628,"inlinee":29089,"source_line":83,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":64504,"parent_record_offset":64476,"ancestor_record_offset":62628,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":72728,"parent_record_offset":72696,"ancestor_record_offset":62628,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":72792,"parent_record_offset":72760,"ancestor_record_offset":62628,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":64532,"parent_record_offset":64504,"ancestor_record_offset":62628,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":72760,"parent_record_offset":72728,"ancestor_record_offset":62628,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":68620,"parent_record_offset":67448,"ancestor_record_offset":62628,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":683,"record_offset":73136,"parent_record_offset":73108,"ancestor_record_offset":62628,"inlinee":29101,"source_line":2840,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":683,"record_offset":64448,"parent_record_offset":64420,"ancestor_record_offset":62628,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":65148,"parent_record_offset":65124,"ancestor_record_offset":62628,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":65172,"parent_record_offset":65148,"ancestor_record_offset":62628,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":72280,"parent_record_offset":72220,"ancestor_record_offset":62628,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":72312,"parent_record_offset":72184,"ancestor_record_offset":62628,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":683,"record_offset":71028,"parent_record_offset":71004,"ancestor_record_offset":62628,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":64124,"parent_record_offset":64096,"ancestor_record_offset":62628,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":64156,"parent_record_offset":64124,"ancestor_record_offset":62628,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":64096,"parent_record_offset":63844,"ancestor_record_offset":62628,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":63956,"parent_record_offset":63844,"ancestor_record_offset":62628,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":69424,"parent_record_offset":69396,"ancestor_record_offset":62628,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":64616,"parent_record_offset":63844,"ancestor_record_offset":62628,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":64040,"parent_record_offset":63844,"ancestor_record_offset":62628,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":683,"record_offset":63900,"parent_record_offset":63844,"ancestor_record_offset":62628,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":683,"record_offset":64392,"parent_record_offset":64364,"ancestor_record_offset":62628,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":63736,"parent_record_offset":63712,"ancestor_record_offset":62628,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":64364,"parent_record_offset":64256,"ancestor_record_offset":62628,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":63768,"parent_record_offset":63736,"ancestor_record_offset":62628,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":69912,"parent_record_offset":69308,"ancestor_record_offset":62628,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":64304,"parent_record_offset":64280,"ancestor_record_offset":62628,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":69764,"parent_record_offset":69544,"ancestor_record_offset":62628,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":64708,"parent_record_offset":63844,"ancestor_record_offset":62628,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":69396,"parent_record_offset":69340,"ancestor_record_offset":62628,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":69544,"parent_record_offset":69516,"ancestor_record_offset":62628,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":64280,"parent_record_offset":64256,"ancestor_record_offset":62628,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":63820,"parent_record_offset":63796,"ancestor_record_offset":62628,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":63796,"parent_record_offset":63736,"ancestor_record_offset":62628,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":70316,"parent_record_offset":70284,"ancestor_record_offset":62628,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":69368,"parent_record_offset":69340,"ancestor_record_offset":62628,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":69308,"parent_record_offset":69248,"ancestor_record_offset":62628,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":69248,"parent_record_offset":69224,"ancestor_record_offset":62628,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":63844,"parent_record_offset":63820,"ancestor_record_offset":62628,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":70284,"parent_record_offset":69248,"ancestor_record_offset":62628,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":69280,"parent_record_offset":69248,"ancestor_record_offset":62628,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":69340,"parent_record_offset":69308,"ancestor_record_offset":62628,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":63928,"parent_record_offset":63844,"ancestor_record_offset":62628,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":70348,"parent_record_offset":70316,"ancestor_record_offset":62628,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":69224,"parent_record_offset":69200,"ancestor_record_offset":62628,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":69516,"parent_record_offset":69308,"ancestor_record_offset":62628,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":64256,"parent_record_offset":63844,"ancestor_record_offset":62628,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":683,"record_offset":63712,"parent_record_offset":63688,"ancestor_record_offset":62628,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":683,"record_offset":74572,"parent_record_offset":74548,"ancestor_record_offset":62628,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":65312,"parent_record_offset":65288,"ancestor_record_offset":62628,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":65288,"parent_record_offset":65264,"ancestor_record_offset":62628,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":65336,"parent_record_offset":65312,"ancestor_record_offset":62628,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":67520,"parent_record_offset":67492,"ancestor_record_offset":62628,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":62856,"parent_record_offset":62828,"ancestor_record_offset":62628,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":62912,"parent_record_offset":62884,"ancestor_record_offset":62628,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":683,"record_offset":63168,"parent_record_offset":63144,"ancestor_record_offset":62628,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":683,"record_offset":63028,"parent_record_offset":63000,"ancestor_record_offset":62628,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":63364,"parent_record_offset":62968,"ancestor_record_offset":62628,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":62884,"parent_record_offset":62856,"ancestor_record_offset":62628,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":683,"record_offset":70888,"parent_record_offset":70864,"ancestor_record_offset":62628,"inlinee":13256,"source_line":181,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":683,"record_offset":70912,"parent_record_offset":70888,"ancestor_record_offset":62628,"inlinee":13258,"source_line":1187,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":683,"record_offset":71004,"parent_record_offset":70980,"ancestor_record_offset":62628,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":683,"record_offset":63056,"parent_record_offset":63028,"ancestor_record_offset":62628,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":683,"record_offset":63088,"parent_record_offset":63056,"ancestor_record_offset":62628,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":683,"record_offset":62968,"parent_record_offset":62940,"ancestor_record_offset":62628,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":683,"record_offset":62940,"parent_record_offset":62912,"ancestor_record_offset":62628,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mod_parts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":73108,"parent_record_offset":73076,"ancestor_record_offset":62628,"inlinee":45553,"source_line":54,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mod_parts::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":67492,"parent_record_offset":67448,"ancestor_record_offset":62628,"inlinee":45552,"source_line":80,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::mod_parts::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1F1
// 首条位置: {"module":683,"inlinee":45553}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":683,"record_offset":73136,"parent_record_offset":73108,"ancestor_record_offset":73108,"inlinee":29101,"source_line":2840,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":683,"record_offset":73260,"parent_record_offset":73220,"ancestor_record_offset":73108,"inlinee":29090,"source_line":694,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_uppercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":683,"record_offset":73588,"parent_record_offset":73424,"ancestor_record_offset":73108,"inlinee":18736,"source_line":816,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":683,"record_offset":73424,"parent_record_offset":73260,"ancestor_record_offset":73108,"inlinee":18735,"source_line":668,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":683,"record_offset":73168,"parent_record_offset":73136,"ancestor_record_offset":73108,"inlinee":29088,"source_line":60,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case_simple
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":683,"record_offset":73220,"parent_record_offset":73168,"ancestor_record_offset":73108,"inlinee":29089,"source_line":83,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::mod_parts::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1F0
// 首条位置: {"module":683,"inlinee":45552}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":67520,"parent_record_offset":67492,"ancestor_record_offset":67492,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":68188,"parent_record_offset":68160,"ancestor_record_offset":67492,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":67748,"parent_record_offset":67724,"ancestor_record_offset":67492,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":67832,"parent_record_offset":67808,"ancestor_record_offset":67492,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":68356,"parent_record_offset":67668,"ancestor_record_offset":67492,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":683,"record_offset":67980,"parent_record_offset":67952,"ancestor_record_offset":67492,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":683,"record_offset":67952,"parent_record_offset":67924,"ancestor_record_offset":67492,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":67772,"parent_record_offset":67748,"ancestor_record_offset":67492,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":68096,"parent_record_offset":68072,"ancestor_record_offset":67492,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":68272,"parent_record_offset":68244,"ancestor_record_offset":67492,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":68300,"parent_record_offset":68272,"ancestor_record_offset":67492,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":683,"record_offset":68216,"parent_record_offset":68188,"ancestor_record_offset":67492,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":67892,"parent_record_offset":67864,"ancestor_record_offset":67492,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":67924,"parent_record_offset":67892,"ancestor_record_offset":67492,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":67864,"parent_record_offset":67668,"ancestor_record_offset":67492,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":67724,"parent_record_offset":67668,"ancestor_record_offset":67492,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":68384,"parent_record_offset":67668,"ancestor_record_offset":67492,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":67808,"parent_record_offset":67668,"ancestor_record_offset":67492,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":683,"record_offset":68508,"parent_record_offset":67668,"ancestor_record_offset":67492,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":683,"record_offset":68160,"parent_record_offset":68132,"ancestor_record_offset":67492,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":67576,"parent_record_offset":67548,"ancestor_record_offset":67492,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":68132,"parent_record_offset":68024,"ancestor_record_offset":67492,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":68576,"parent_record_offset":67576,"ancestor_record_offset":67492,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":68072,"parent_record_offset":68048,"ancestor_record_offset":67492,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":68476,"parent_record_offset":67668,"ancestor_record_offset":67492,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":68048,"parent_record_offset":68024,"ancestor_record_offset":67492,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":67640,"parent_record_offset":67612,"ancestor_record_offset":67492,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":67612,"parent_record_offset":67576,"ancestor_record_offset":67492,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":67668,"parent_record_offset":67640,"ancestor_record_offset":67492,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":68536,"parent_record_offset":67668,"ancestor_record_offset":67492,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":68024,"parent_record_offset":67668,"ancestor_record_offset":67492,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":683,"record_offset":67548,"parent_record_offset":67520,"ancestor_record_offset":67492,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::has_exact_entry
// 记录形式: compiled_procedure; instances=1
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":49328,"rva":14236304,"va":5382945424}
// 编译实例 RVAs: 0xd93a90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1743821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945424,"raw_hex":"e8cd9b1a00","size":5,"soft_fail":false,"target_va":5384689584,"va":5382945758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1822636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945424,"raw_hex":"e8accf1b00","size":5,"soft_fail":false,"target_va":5384768304,"va":5382945663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1818485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945424,"raw_hex":"e875bf1b00","size":5,"soft_fail":false,"target_va":5384764464,"va":5382945974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7267126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945424,"raw_hex":"e8ca1c91ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382945825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13611128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945424,"raw_hex":"e8884f30ff","size":5,"soft_fail":false,"target_va":5369335008,"va":5382946131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13606715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945424,"raw_hex":"e8c56030ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5382945558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34111515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945424,"raw_hex":"e81b800802","size":5,"soft_fail":false,"target_va":5417057056,"va":5382945536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-823059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945424,"raw_hex":"e8ed70f3ff","size":5,"soft_fail":false,"target_va":5382122800,"va":5382945854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1779672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945424,"raw_hex":"e8d8271b00","size":5,"soft_fail":false,"target_va":5384725152,"va":5382945475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":49540,"parent_record_offset":49328,"ancestor_record_offset":49328,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":49516,"parent_record_offset":49328,"ancestor_record_offset":49328,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":49456,"parent_record_offset":49328,"ancestor_record_offset":49328,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":50232,"parent_record_offset":50208,"ancestor_record_offset":49328,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":50256,"parent_record_offset":50232,"ancestor_record_offset":49328,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":50280,"parent_record_offset":50256,"ancestor_record_offset":49328,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":50208,"parent_record_offset":50152,"ancestor_record_offset":49328,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":50180,"parent_record_offset":50152,"ancestor_record_offset":49328,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":50152,"parent_record_offset":50128,"ancestor_record_offset":49328,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":50128,"parent_record_offset":50104,"ancestor_record_offset":49328,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":50456,"parent_record_offset":50420,"ancestor_record_offset":49328,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":50420,"parent_record_offset":50396,"ancestor_record_offset":49328,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":49848,"parent_record_offset":49820,"ancestor_record_offset":49328,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":683,"record_offset":49632,"parent_record_offset":49596,"ancestor_record_offset":49328,"inlinee":5470,"source_line":49,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":683,"record_offset":49568,"parent_record_offset":49540,"ancestor_record_offset":49328,"inlinee":6890,"source_line":132,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":49764,"parent_record_offset":49632,"ancestor_record_offset":49328,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":49596,"parent_record_offset":49568,"ancestor_record_offset":49328,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":49668,"parent_record_offset":49632,"ancestor_record_offset":49328,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":49728,"parent_record_offset":49696,"ancestor_record_offset":49328,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":50488,"parent_record_offset":50456,"ancestor_record_offset":49328,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":51272,"parent_record_offset":49792,"ancestor_record_offset":49328,"inlinee":36640,"source_line":619,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":49480,"parent_record_offset":49456,"ancestor_record_offset":49328,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":49696,"parent_record_offset":49668,"ancestor_record_offset":49328,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":49932,"parent_record_offset":49904,"ancestor_record_offset":49328,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":49960,"parent_record_offset":49932,"ancestor_record_offset":49328,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":50516,"parent_record_offset":50456,"ancestor_record_offset":49328,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":50548,"parent_record_offset":50420,"ancestor_record_offset":49328,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$13::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":49820,"parent_record_offset":49792,"ancestor_record_offset":49328,"inlinee":45548,"source_line":731,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$44::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":49904,"parent_record_offset":49876,"ancestor_record_offset":49328,"inlinee":14923,"source_line":1498,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$45::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":49876,"parent_record_offset":49848,"ancestor_record_offset":49328,"inlinee":33658,"source_line":1506,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":51176,"parent_record_offset":49792,"ancestor_record_offset":49328,"inlinee":13254,"source_line":2610,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":51320,"parent_record_offset":51296,"ancestor_record_offset":49328,"inlinee":33659,"source_line":2425,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::is_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":683,"record_offset":51344,"parent_record_offset":51320,"ancestor_record_offset":49328,"inlinee":33660,"source_line":1199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":683,"record_offset":51232,"parent_record_offset":51200,"ancestor_record_offset":49328,"inlinee":13258,"source_line":1187,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::has_exact_entry::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":49792,"parent_record_offset":49764,"ancestor_record_offset":49328,"inlinee":45546,"source_line":94,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::has_exact_entry::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":51296,"parent_record_offset":51272,"ancestor_record_offset":49328,"inlinee":45550,"source_line":94,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::has_exact_entry::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1EA
// 首条位置: {"module":683,"inlinee":45546}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":50008,"parent_record_offset":49792,"ancestor_record_offset":49792,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":51272,"parent_record_offset":49792,"ancestor_record_offset":49792,"inlinee":36640,"source_line":619,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::impl$13::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":49820,"parent_record_offset":49792,"ancestor_record_offset":49792,"inlinee":45548,"source_line":731,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":51176,"parent_record_offset":49792,"ancestor_record_offset":49792,"inlinee":13254,"source_line":2610,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":50232,"parent_record_offset":50208,"ancestor_record_offset":49792,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":50256,"parent_record_offset":50232,"ancestor_record_offset":49792,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":50280,"parent_record_offset":50256,"ancestor_record_offset":49792,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":50208,"parent_record_offset":50152,"ancestor_record_offset":49792,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":50180,"parent_record_offset":50152,"ancestor_record_offset":49792,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":50152,"parent_record_offset":50128,"ancestor_record_offset":49792,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":50128,"parent_record_offset":50104,"ancestor_record_offset":49792,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":50456,"parent_record_offset":50420,"ancestor_record_offset":49792,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":50420,"parent_record_offset":50396,"ancestor_record_offset":49792,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":49848,"parent_record_offset":49820,"ancestor_record_offset":49792,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":50032,"parent_record_offset":50008,"ancestor_record_offset":49792,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":50488,"parent_record_offset":50456,"ancestor_record_offset":49792,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":51200,"parent_record_offset":51176,"ancestor_record_offset":49792,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":49932,"parent_record_offset":49904,"ancestor_record_offset":49792,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":49960,"parent_record_offset":49932,"ancestor_record_offset":49792,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":50516,"parent_record_offset":50456,"ancestor_record_offset":49792,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":50548,"parent_record_offset":50420,"ancestor_record_offset":49792,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$44::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":49904,"parent_record_offset":49876,"ancestor_record_offset":49792,"inlinee":14923,"source_line":1498,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$45::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":49876,"parent_record_offset":49848,"ancestor_record_offset":49792,"inlinee":33658,"source_line":1506,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":51320,"parent_record_offset":51296,"ancestor_record_offset":49792,"inlinee":33659,"source_line":2425,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::is_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":683,"record_offset":51344,"parent_record_offset":51320,"ancestor_record_offset":49792,"inlinee":33660,"source_line":1199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":683,"record_offset":51232,"parent_record_offset":51200,"ancestor_record_offset":49792,"inlinee":13258,"source_line":1187,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::has_exact_entry::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":51296,"parent_record_offset":51272,"ancestor_record_offset":49792,"inlinee":45550,"source_line":94,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::has_exact_entry::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1EE
// 首条位置: {"module":683,"inlinee":45550}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::fs::FileType::is_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":51320,"parent_record_offset":51296,"ancestor_record_offset":51296,"inlinee":33659,"source_line":2425,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::is_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":683,"record_offset":51344,"parent_record_offset":51320,"ancestor_record_offset":51296,"inlinee":33660,"source_line":1199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::deleted_mods_root
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1C9
// 首条位置: {"module":683,"inlinee":45513}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":2288,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":2376,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":2468,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::mods_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2092,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":45514,"source_line":13,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":2032,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":2032,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":2032,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":2728,"parent_record_offset":2664,"ancestor_record_offset":2032,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":2696,"parent_record_offset":2664,"ancestor_record_offset":2032,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":2664,"parent_record_offset":2636,"ancestor_record_offset":2032,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":2032,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":2156,"parent_record_offset":2092,"ancestor_record_offset":2032,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":2192,"parent_record_offset":2092,"ancestor_record_offset":2032,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":2496,"parent_record_offset":2468,"ancestor_record_offset":2032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::env::var_os
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\env.rs
//     evidence={"module":683,"record_offset":2128,"parent_record_offset":2092,"ancestor_record_offset":2032,"inlinee":10543,"source_line":259,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":2032,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::deleted_mods_root::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2316,"parent_record_offset":2288,"ancestor_record_offset":2032,"inlinee":45518,"source_line":103,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::deleted_mods_root::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2408,"parent_record_offset":2376,"ancestor_record_offset":2032,"inlinee":45519,"source_line":104,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mods_root::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2224,"parent_record_offset":2192,"ancestor_record_offset":2032,"inlinee":45516,"source_line":16,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::deleted_mods_root::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1CE
// 首条位置: {"module":683,"inlinee":45518}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::deleted_mods_root::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1CF
// 首条位置: {"module":683,"inlinee":45519}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::shelve_mod
// 记录形式: compiled_procedure; instances=1
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":1744,"rva":14217408,"va":5382926528}
// 编译实例 RVAs: 0xd8f0c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1732077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8ed6d1a00","size":5,"soft_fail":false,"target_va":5384659264,"va":5382927182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1762741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8b5e51a00","size":5,"soft_fail":false,"target_va":5384689616,"va":5382926870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1729535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8ff631a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5382926700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1730439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e887671a00","size":5,"soft_fail":false,"target_va":5384657024,"va":5382926580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1729765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8e5641a00","size":5,"soft_fail":false,"target_va":5384656512,"va":5382926742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-7248150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8ea6691ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382926849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db0610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16913288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e888130201","size":5,"soft_fail":false,"target_va":5399840272,"va":5382926979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db0ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16914887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8c7190201","size":5,"soft_fail":false,"target_va":5399841952,"va":5382927060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h796235da11270d01 (.llvm.17100869928342201643)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33923545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8d9a10502","size":5,"soft_fail":false,"target_va":5416850752,"va":5382927202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33924503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e897a50502","size":5,"soft_fail":false,"target_va":5416851392,"va":5382926884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33694283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e84b220202","size":5,"soft_fail":false,"target_va":5416621536,"va":5382927248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1903471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e86f0b1d00","size":5,"soft_fail":false,"target_va":5384830080,"va":5382926604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t212894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e89e3f0300","size":5,"soft_fail":false,"target_va":5383139568,"va":5382926669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var_os
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1785647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e82f3f1b00","size":5,"soft_fail":false,"target_va":5384712304,"va":5382926652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::rename
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1797400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382926528,"raw_hex":"e8186d1b00","size":5,"soft_fail":false,"target_va":5384724448,"va":5382927043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":683,"record_offset":4356,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":3808,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":3752,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":1896,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":3868,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":3384,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":3356,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":3244,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::rename
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":3904,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":13844,"source_line":2818,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::deleted_mods_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2032,"parent_record_offset":1744,"ancestor_record_offset":1744,"inlinee":45513,"source_line":100,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":1744,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":1744,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":1744,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":2728,"parent_record_offset":2664,"ancestor_record_offset":1744,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":2696,"parent_record_offset":2664,"ancestor_record_offset":1744,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":2664,"parent_record_offset":2636,"ancestor_record_offset":1744,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":1744,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":683,"record_offset":4392,"parent_record_offset":4356,"ancestor_record_offset":1744,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":683,"record_offset":3296,"parent_record_offset":3244,"ancestor_record_offset":1744,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":683,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":1744,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":683,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":1744,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":2156,"parent_record_offset":2092,"ancestor_record_offset":1744,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":2192,"parent_record_offset":2092,"ancestor_record_offset":1744,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":2468,"parent_record_offset":2032,"ancestor_record_offset":1744,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":683,"record_offset":1988,"parent_record_offset":1964,"ancestor_record_offset":1744,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::env::var_os
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\env.rs
//     evidence={"module":683,"record_offset":2128,"parent_record_offset":2092,"ancestor_record_offset":1744,"inlinee":10543,"source_line":259,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":1940,"parent_record_offset":1920,"ancestor_record_offset":1744,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":3324,"parent_record_offset":3244,"ancestor_record_offset":1744,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":3268,"parent_record_offset":3244,"ancestor_record_offset":1744,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":1744,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":683,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":1744,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::deleted_mods_root::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2316,"parent_record_offset":2288,"ancestor_record_offset":1744,"inlinee":45518,"source_line":103,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::deleted_mods_root::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2408,"parent_record_offset":2376,"ancestor_record_offset":1744,"inlinee":45519,"source_line":104,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mods_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2092,"parent_record_offset":2032,"ancestor_record_offset":1744,"inlinee":45514,"source_line":13,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mods_root::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":2224,"parent_record_offset":2192,"ancestor_record_offset":1744,"inlinee":45516,"source_line":16,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::shelve_mod::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":1920,"parent_record_offset":1896,"ancestor_record_offset":1744,"inlinee":45511,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::shelve_mod::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1C7
// 首条位置: {"module":683,"inlinee":45511}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":1940,"parent_record_offset":1920,"ancestor_record_offset":1920,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":683,"record_offset":1988,"parent_record_offset":1964,"ancestor_record_offset":1920,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":683,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":1920,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::shelve_mod::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1748D
// 首条位置: {"module":2045,"inlinee":95373}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2045,"record_offset":10628,"parent_record_offset":10568,"ancestor_record_offset":10568,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":2045,"record_offset":10600,"parent_record_offset":10568,"ancestor_record_offset":10568,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2045,"record_offset":10924,"parent_record_offset":10900,"ancestor_record_offset":10568,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2045,"record_offset":10948,"parent_record_offset":10924,"ancestor_record_offset":10568,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2045,"record_offset":10972,"parent_record_offset":10948,"ancestor_record_offset":10568,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2045,"record_offset":10900,"parent_record_offset":10844,"ancestor_record_offset":10568,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2045,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":10568,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2045,"record_offset":10872,"parent_record_offset":10844,"ancestor_record_offset":10568,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2045,"record_offset":10844,"parent_record_offset":10820,"ancestor_record_offset":10568,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2045,"record_offset":10820,"parent_record_offset":10796,"ancestor_record_offset":10568,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2045,"record_offset":10652,"parent_record_offset":10628,"ancestor_record_offset":10568,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2045,"record_offset":10748,"parent_record_offset":10712,"ancestor_record_offset":10568,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::shelve_mod::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1748E
// 首条位置: {"module":2045,"inlinee":95374}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":2045,"record_offset":11388,"parent_record_offset":11364,"ancestor_record_offset":11364,"inlinee":14220,"source_line":3443,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2045,"record_offset":11480,"parent_record_offset":11388,"ancestor_record_offset":11364,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2045,"record_offset":11440,"parent_record_offset":11412,"ancestor_record_offset":11364,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":2045,"record_offset":11412,"parent_record_offset":11388,"ancestor_record_offset":11364,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::shelve_mod::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17496
// 首条位置: {"module":2046,"inlinee":95382}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2046,"record_offset":1496,"parent_record_offset":1476,"ancestor_record_offset":1476,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2046,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":1476,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02046.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2046,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":1476,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::partless_mod_folders
// 记录形式: compiled_procedure; instances=1
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":61044,"rva":14239744,"va":5382948864}
// 编译实例 RVAs: 0xd94800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1707268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382948864,"raw_hex":"e8040d1a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5382948967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7270424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382948864,"raw_hex":"e8e80f91ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382949123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13679154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382948864,"raw_hex":"e8ce452fff","size":5,"soft_fail":false,"target_va":5369270288,"va":5382949437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13610301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382948864,"raw_hex":"e8c35230ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5382949144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da6270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16849170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382948864,"raw_hex":"e812190101","size":5,"soft_fail":false,"target_va":5399798384,"va":5382949209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dac880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16875212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382948864,"raw_hex":"e8cc7e0101","size":5,"soft_fail":false,"target_va":5399824512,"va":5382949295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34107958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382948864,"raw_hex":"e836720802","size":5,"soft_fail":false,"target_va":5417057056,"va":5382949093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16886238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382948864,"raw_hex":"e8dea90101","size":5,"soft_fail":false,"target_va":5399835568,"va":5382949325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t190630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382948864,"raw_hex":"e8a6e80200","size":5,"soft_fail":false,"target_va":5383139568,"va":5382948933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var_os
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1763383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382948864,"raw_hex":"e837e81a00","size":5,"soft_fail":false,"target_va":5384712304,"va":5382948916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1776119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382948864,"raw_hex":"e8f7191b00","size":5,"soft_fail":false,"target_va":5384725152,"va":5382949028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":62340,"parent_record_offset":61044,"ancestor_record_offset":61044,"inlinee":11475,"source_line":131,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":62256,"parent_record_offset":61044,"ancestor_record_offset":61044,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":62284,"parent_record_offset":61044,"ancestor_record_offset":61044,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":62228,"parent_record_offset":61044,"ancestor_record_offset":61044,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":61372,"parent_record_offset":61044,"ancestor_record_offset":61044,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::mods_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":61188,"parent_record_offset":61044,"ancestor_record_offset":61044,"inlinee":45514,"source_line":13,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":61736,"parent_record_offset":61712,"ancestor_record_offset":61044,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":61760,"parent_record_offset":61736,"ancestor_record_offset":61044,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":61784,"parent_record_offset":61760,"ancestor_record_offset":61044,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":61712,"parent_record_offset":61648,"ancestor_record_offset":61044,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":61680,"parent_record_offset":61648,"ancestor_record_offset":61044,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":61648,"parent_record_offset":61620,"ancestor_record_offset":61044,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":61620,"parent_record_offset":61592,"ancestor_record_offset":61044,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":62368,"parent_record_offset":62340,"ancestor_record_offset":61044,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":62308,"parent_record_offset":62284,"ancestor_record_offset":61044,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":62432,"parent_record_offset":62396,"ancestor_record_offset":61044,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":61252,"parent_record_offset":61188,"ancestor_record_offset":61044,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":61284,"parent_record_offset":61188,"ancestor_record_offset":61044,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":61452,"parent_record_offset":61372,"ancestor_record_offset":61044,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":62524,"parent_record_offset":62488,"ancestor_record_offset":61044,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":61416,"parent_record_offset":61372,"ancestor_record_offset":61044,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":683,"record_offset":62488,"parent_record_offset":62396,"ancestor_record_offset":61044,"inlinee":9500,"source_line":580,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":683,"record_offset":62396,"parent_record_offset":62368,"ancestor_record_offset":61044,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::env::var_os
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\env.rs
//     evidence={"module":683,"record_offset":61224,"parent_record_offset":61188,"ancestor_record_offset":61044,"inlinee":10543,"source_line":259,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":61332,"parent_record_offset":61312,"ancestor_record_offset":61044,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mods_root::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":61312,"parent_record_offset":61284,"ancestor_record_offset":61044,"inlinee":45516,"source_line":16,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::partless_mod_folders::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1748C
// 首条位置: {"module":2045,"inlinee":95372}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2045,"record_offset":8560,"parent_record_offset":8532,"ancestor_record_offset":8532,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2045,"record_offset":8668,"parent_record_offset":8632,"ancestor_record_offset":8532,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2045,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":8532,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2045,"record_offset":8584,"parent_record_offset":8560,"ancestor_record_offset":8532,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2045,"record_offset":8700,"parent_record_offset":8668,"ancestor_record_offset":8532,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2045,"record_offset":8728,"parent_record_offset":8668,"ancestor_record_offset":8532,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2045,"record_offset":8760,"parent_record_offset":8632,"ancestor_record_offset":8532,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::partless_mod_folders::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F7D4
// 首条位置: {"module":3559,"inlinee":128980}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3559,"record_offset":1224,"parent_record_offset":484,"ancestor_record_offset":484,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":3559,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":484,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::overlay::ModParts::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/overlay.rs
//     evidence={"module":3559,"record_offset":1188,"parent_record_offset":484,"ancestor_record_offset":484,"inlinee":45506,"source_line":29,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3559,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":484,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3559,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":484,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3559,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":484,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3559,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":484,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3559,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":484,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3559,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":484,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3559,"record_offset":948,"parent_record_offset":800,"ancestor_record_offset":484,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3559,"record_offset":1512,"parent_record_offset":1452,"ancestor_record_offset":484,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3559,"record_offset":740,"parent_record_offset":708,"ancestor_record_offset":484,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3559,"record_offset":1484,"parent_record_offset":1452,"ancestor_record_offset":484,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3559,"record_offset":1452,"parent_record_offset":1424,"ancestor_record_offset":484,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3559,"record_offset":800,"parent_record_offset":768,"ancestor_record_offset":484,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3559,"record_offset":768,"parent_record_offset":740,"ancestor_record_offset":484,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3559,"record_offset":1424,"parent_record_offset":1396,"ancestor_record_offset":484,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3559,"record_offset":836,"parent_record_offset":800,"ancestor_record_offset":484,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3559,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":484,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3559,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":484,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3559,"record_offset":1132,"parent_record_offset":672,"ancestor_record_offset":484,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3559,"record_offset":708,"parent_record_offset":672,"ancestor_record_offset":484,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3559,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":484,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":3559,"record_offset":592,"parent_record_offset":564,"ancestor_record_offset":484,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":3559,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":484,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3559,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":484,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3559,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":484,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3559,"record_offset":1076,"parent_record_offset":672,"ancestor_record_offset":484,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3559,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":484,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3559,"record_offset":1256,"parent_record_offset":1224,"ancestor_record_offset":484,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":3559,"record_offset":536,"parent_record_offset":508,"ancestor_record_offset":484,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":3559,"record_offset":564,"parent_record_offset":536,"ancestor_record_offset":484,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::discover_mods
// 记录形式: compiled_procedure; instances=1
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":10060,"rva":14220176,"va":5382929296}
// 编译实例 RVAs: 0xd8fb90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1759012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e824d71a00","size":5,"soft_fail":false,"target_va":5384689312,"va":5382930295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1838092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e80c0c1c00","size":5,"soft_fail":false,"target_va":5384768304,"va":5382930207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1715404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8cc2c1a00","size":5,"soft_fail":false,"target_va":5384644912,"va":5382929503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1726790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e846591a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5382929445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_dir
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1726080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e880561a00","size":5,"soft_fail":false,"target_va":5384656400,"va":5382930315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1833396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8b4f91b00","size":5,"soft_fail":false,"target_va":5384764464,"va":5382931063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::SystemTime>::elapsed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1730633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e849681a00","size":5,"soft_fail":false,"target_va":5384661248,"va":5382930610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$T$u20$as$u20$core..slice..cmp..SliceContains$GT$::slice_contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t16873925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8c5790101","size":5,"soft_fail":false,"target_va":5399804272,"va":5382930342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1677827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8039a1900","size":5,"soft_fail":false,"target_va":5384609408,"va":5382931576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=20
//     evidence={"assembly":"\tcallq\t-7251143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8395b91ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382929842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087a30
//     owner=unresolved_namespace; count=2
//     target_aliases=49 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13669419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8d56b2fff","size":5,"soft_fail":false,"target_va":5369264688,"va":5382934102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13664347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8a57f2fff","size":5,"soft_fail":false,"target_va":5369270288,"va":5382934630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13598427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8258130ff","size":5,"soft_fail":false,"target_va":5369335008,"va":5382933430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=8
//     target_aliases=3 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13591277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8139d30ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5382930120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13503387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e865f431ff","size":5,"soft_fail":false,"target_va":5369431360,"va":5382934742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b05c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13503159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e849f531ff","size":5,"soft_fail":false,"target_va":5369431488,"va":5382934642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9451422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e862c86fff","size":5,"soft_fail":false,"target_va":5373480896,"va":5382932313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8832071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8b93b79ff","size":5,"soft_fail":false,"target_va":5374098704,"va":5382930770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4201291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8b5e4bfff","size":5,"soft_fail":false,"target_va":5378733248,"va":5382934534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da6970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16868301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8cd630101","size":5,"soft_fail":false,"target_va":5399800176,"va":5382931870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dafe60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16906304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e840f80101","size":5,"soft_fail":false,"target_va":5399838304,"va":5382931995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34124899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e863b40802","size":5,"soft_fail":false,"target_va":5417057040,"va":5382932136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34126982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e886bc0802","size":5,"soft_fail":false,"target_va":5417057056,"va":5382930069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-807605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e84badf3ff","size":5,"soft_fail":false,"target_va":5382122800,"va":5382930400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t210152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8e8340300","size":5,"soft_fail":false,"target_va":5383139568,"va":5382929411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::standard_app_root_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t208853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8d52f0300","size":5,"soft_fail":false,"target_va":5383138784,"va":5382929926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var_os
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1782908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e87c341b00","size":5,"soft_fail":false,"target_va":5384712304,"va":5382929391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1794423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e877611b00","size":5,"soft_fail":false,"target_va":5384724912,"va":5382930484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1795152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e850641b00","size":5,"soft_fail":false,"target_va":5384725152,"va":5382929995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::remove_dir_all
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1792437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8b5591b00","size":5,"soft_fail":false,"target_va":5384723232,"va":5382930790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::remove_file
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1790107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e89b501b00","size":5,"soft_fail":false,"target_va":5384722736,"va":5382932624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1793598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e83e5e1b00","size":5,"soft_fail":false,"target_va":5384726688,"va":5382933085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::discovery::scan_mods_root
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e893140000","size":5,"soft_fail":false,"target_va":5382934880,"va":5382929608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::identity::sidecar_zip_stem
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14397093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e8a5aedb00","size":5,"soft_fail":false,"target_va":5397329360,"va":5382932262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::ModParts::describe
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382929296,"raw_hex":"e880d8ffff","size":5,"soft_fail":false,"target_va":5382921216,"va":5382931323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":22544,"parent_record_offset":10060,"ancestor_record_offset":10060,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":22832,"parent_record_offset":10060,"ancestor_record_offset":10060,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":11116,"parent_record_offset":10060,"ancestor_record_offset":10060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":22656,"parent_record_offset":10060,"ancestor_record_offset":10060,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":22888,"parent_record_offset":10060,"ancestor_record_offset":10060,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":22748,"parent_record_offset":10060,"ancestor_record_offset":10060,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":10400,"parent_record_offset":10060,"ancestor_record_offset":10060,"inlinee":5639,"source_line":149,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::mods_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":10216,"parent_record_offset":10060,"ancestor_record_offset":10060,"inlinee":45514,"source_line":13,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::trim_mod_cache
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":12616,"parent_record_offset":10060,"ancestor_record_offset":10060,"inlinee":45526,"source_line":532,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::trim_orphaned_zip_sidecars
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":17616,"parent_record_offset":10060,"ancestor_record_offset":10060,"inlinee":45530,"source_line":444,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":12148,"parent_record_offset":12124,"ancestor_record_offset":10060,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":12172,"parent_record_offset":12148,"ancestor_record_offset":10060,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":10060,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":12124,"parent_record_offset":12068,"ancestor_record_offset":10060,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":12096,"parent_record_offset":12068,"ancestor_record_offset":10060,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":12068,"parent_record_offset":12044,"ancestor_record_offset":10060,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":10060,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":14424,"parent_record_offset":14388,"ancestor_record_offset":10060,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":14388,"parent_record_offset":14364,"ancestor_record_offset":10060,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":17860,"parent_record_offset":17836,"ancestor_record_offset":10060,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":21544,"parent_record_offset":21520,"ancestor_record_offset":10060,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":683,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":10060,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":683,"record_offset":10868,"parent_record_offset":10664,"ancestor_record_offset":10060,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":15600,"parent_record_offset":12616,"ancestor_record_offset":10060,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":19532,"parent_record_offset":19508,"ancestor_record_offset":10060,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":683,"record_offset":15680,"parent_record_offset":15656,"ancestor_record_offset":10060,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":11436,"parent_record_offset":11412,"ancestor_record_offset":10060,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":19100,"parent_record_offset":19076,"ancestor_record_offset":10060,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":19184,"parent_record_offset":19160,"ancestor_record_offset":10060,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":19700,"parent_record_offset":19020,"ancestor_record_offset":10060,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":683,"record_offset":13668,"parent_record_offset":12616,"ancestor_record_offset":10060,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":17836,"parent_record_offset":17616,"ancestor_record_offset":10060,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":13692,"parent_record_offset":13668,"ancestor_record_offset":10060,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":13748,"parent_record_offset":13716,"ancestor_record_offset":10060,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":18736,"parent_record_offset":18712,"ancestor_record_offset":10060,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":13716,"parent_record_offset":13692,"ancestor_record_offset":10060,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":683,"record_offset":10832,"parent_record_offset":10808,"ancestor_record_offset":10060,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":12364,"parent_record_offset":12336,"ancestor_record_offset":10060,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":12392,"parent_record_offset":12364,"ancestor_record_offset":10060,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":22012,"parent_record_offset":21984,"ancestor_record_offset":10060,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":10996,"parent_record_offset":10940,"ancestor_record_offset":10060,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":683,"record_offset":19332,"parent_record_offset":19304,"ancestor_record_offset":10060,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":683,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":10060,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":13804,"parent_record_offset":13776,"ancestor_record_offset":10060,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":10576,"parent_record_offset":10540,"ancestor_record_offset":10060,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":13776,"parent_record_offset":13748,"ancestor_record_offset":10060,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":15624,"parent_record_offset":15600,"ancestor_record_offset":10060,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":10280,"parent_record_offset":10216,"ancestor_record_offset":10060,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":10312,"parent_record_offset":10216,"ancestor_record_offset":10060,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":683,"record_offset":11468,"parent_record_offset":11384,"ancestor_record_offset":10060,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":19124,"parent_record_offset":19100,"ancestor_record_offset":10060,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=112
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":11140,"parent_record_offset":11116,"ancestor_record_offset":10060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":18832,"parent_record_offset":18808,"ancestor_record_offset":10060,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":11924,"parent_record_offset":11900,"ancestor_record_offset":10060,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":10784,"parent_record_offset":10664,"ancestor_record_offset":10060,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":11776,"parent_record_offset":11752,"ancestor_record_offset":10060,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":22716,"parent_record_offset":22656,"ancestor_record_offset":10060,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":14456,"parent_record_offset":14424,"ancestor_record_offset":10060,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":19984,"parent_record_offset":19960,"ancestor_record_offset":10060,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":10808,"parent_record_offset":10784,"ancestor_record_offset":10060,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":12828,"parent_record_offset":12788,"ancestor_record_offset":10060,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":13832,"parent_record_offset":13804,"ancestor_record_offset":10060,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":15400,"parent_record_offset":12616,"ancestor_record_offset":10060,"inlinee":15644,"source_line":1559,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":19616,"parent_record_offset":19588,"ancestor_record_offset":10060,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":19644,"parent_record_offset":19616,"ancestor_record_offset":10060,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":683,"record_offset":13920,"parent_record_offset":12616,"ancestor_record_offset":10060,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":683,"record_offset":18784,"parent_record_offset":17616,"ancestor_record_offset":10060,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":683,"record_offset":18808,"parent_record_offset":18784,"ancestor_record_offset":10060,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":19960,"parent_record_offset":17616,"ancestor_record_offset":10060,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":683,"record_offset":19560,"parent_record_offset":19532,"ancestor_record_offset":10060,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":15540,"parent_record_offset":15516,"ancestor_record_offset":10060,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":15564,"parent_record_offset":15540,"ancestor_record_offset":10060,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":14484,"parent_record_offset":14424,"ancestor_record_offset":10060,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":14516,"parent_record_offset":14388,"ancestor_record_offset":10060,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":11600,"parent_record_offset":11572,"ancestor_record_offset":10060,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":11632,"parent_record_offset":11600,"ancestor_record_offset":10060,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":11572,"parent_record_offset":11532,"ancestor_record_offset":10060,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":19076,"parent_record_offset":19020,"ancestor_record_offset":10060,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":11412,"parent_record_offset":11384,"ancestor_record_offset":10060,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":19728,"parent_record_offset":19020,"ancestor_record_offset":10060,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":19160,"parent_record_offset":19020,"ancestor_record_offset":10060,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":683,"record_offset":19852,"parent_record_offset":19020,"ancestor_record_offset":10060,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":683,"record_offset":19508,"parent_record_offset":19484,"ancestor_record_offset":10060,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":18924,"parent_record_offset":18896,"ancestor_record_offset":10060,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":19484,"parent_record_offset":19376,"ancestor_record_offset":10060,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":18712,"parent_record_offset":18688,"ancestor_record_offset":10060,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":19920,"parent_record_offset":18924,"ancestor_record_offset":10060,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":11900,"parent_record_offset":11296,"ancestor_record_offset":10060,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":19424,"parent_record_offset":19400,"ancestor_record_offset":10060,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":11752,"parent_record_offset":11532,"ancestor_record_offset":10060,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":19820,"parent_record_offset":19020,"ancestor_record_offset":10060,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":11384,"parent_record_offset":11328,"ancestor_record_offset":10060,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":11532,"parent_record_offset":11504,"ancestor_record_offset":10060,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":19400,"parent_record_offset":19376,"ancestor_record_offset":10060,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":18992,"parent_record_offset":18964,"ancestor_record_offset":10060,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":18964,"parent_record_offset":18924,"ancestor_record_offset":10060,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":12304,"parent_record_offset":12272,"ancestor_record_offset":10060,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":11356,"parent_record_offset":11328,"ancestor_record_offset":10060,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":11296,"parent_record_offset":11236,"ancestor_record_offset":10060,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":11236,"parent_record_offset":11212,"ancestor_record_offset":10060,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":19020,"parent_record_offset":18992,"ancestor_record_offset":10060,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":12272,"parent_record_offset":11236,"ancestor_record_offset":10060,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":11268,"parent_record_offset":11236,"ancestor_record_offset":10060,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":11328,"parent_record_offset":11296,"ancestor_record_offset":10060,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":19880,"parent_record_offset":19020,"ancestor_record_offset":10060,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":12336,"parent_record_offset":12304,"ancestor_record_offset":10060,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":11212,"parent_record_offset":11188,"ancestor_record_offset":10060,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":11504,"parent_record_offset":11296,"ancestor_record_offset":10060,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":19376,"parent_record_offset":19020,"ancestor_record_offset":10060,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":683,"record_offset":18896,"parent_record_offset":18868,"ancestor_record_offset":10060,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":683,"record_offset":18688,"parent_record_offset":18664,"ancestor_record_offset":10060,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":20260,"parent_record_offset":20236,"ancestor_record_offset":10060,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":20212,"parent_record_offset":17616,"ancestor_record_offset":10060,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":20236,"parent_record_offset":20212,"ancestor_record_offset":10060,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":20284,"parent_record_offset":20260,"ancestor_record_offset":10060,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":15516,"parent_record_offset":12616,"ancestor_record_offset":10060,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":18868,"parent_record_offset":17616,"ancestor_record_offset":10060,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":18636,"parent_record_offset":17932,"ancestor_record_offset":10060,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":18664,"parent_record_offset":17932,"ancestor_record_offset":10060,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":10428,"parent_record_offset":10400,"ancestor_record_offset":10060,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":17932,"parent_record_offset":17892,"ancestor_record_offset":10060,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::env::var_os
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\env.rs
//     evidence={"module":683,"record_offset":10252,"parent_record_offset":10216,"ancestor_record_offset":10060,"inlinee":10543,"source_line":259,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":15452,"parent_record_offset":15428,"ancestor_record_offset":10060,"inlinee":18112,"source_line":2575,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":14588,"parent_record_offset":12616,"ancestor_record_offset":10060,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":12788,"parent_record_offset":12616,"ancestor_record_offset":10060,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":15656,"parent_record_offset":12616,"ancestor_record_offset":10060,"inlinee":13843,"source_line":3232,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":20324,"parent_record_offset":17616,"ancestor_record_offset":10060,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":10484,"parent_record_offset":10456,"ancestor_record_offset":10060,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":683,"record_offset":10744,"parent_record_offset":10720,"ancestor_record_offset":10060,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":683,"record_offset":10604,"parent_record_offset":10576,"ancestor_record_offset":10060,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":10940,"parent_record_offset":10540,"ancestor_record_offset":10060,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":10456,"parent_record_offset":10428,"ancestor_record_offset":10060,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":10360,"parent_record_offset":10340,"ancestor_record_offset":10060,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":683,"record_offset":15476,"parent_record_offset":15452,"ancestor_record_offset":10060,"inlinee":18113,"source_line":188,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":683,"record_offset":10632,"parent_record_offset":10604,"ancestor_record_offset":10060,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":683,"record_offset":10664,"parent_record_offset":10632,"ancestor_record_offset":10060,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":683,"record_offset":10540,"parent_record_offset":10512,"ancestor_record_offset":10060,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":683,"record_offset":10512,"parent_record_offset":10484,"ancestor_record_offset":10060,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mod_cache_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":12728,"parent_record_offset":12616,"ancestor_record_offset":10060,"inlinee":45527,"source_line":21,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mods_root::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":10340,"parent_record_offset":10312,"ancestor_record_offset":10060,"inlinee":45516,"source_line":16,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::trim_mod_cache::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":15428,"parent_record_offset":15400,"ancestor_record_offset":10060,"inlinee":45529,"source_line":545,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::rediscover_mods
// 记录形式: compiled_procedure; instances=1
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":52192,"rva":14237120,"va":5382946240}
// 编译实例 RVAs: 0xd93dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1698577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e811eb1900","size":5,"soft_fail":false,"target_va":5384644912,"va":5382946330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1709719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e897161a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5382946516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-7267948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e8941991ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382946647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13677042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e80e4e2fff","size":5,"soft_fail":false,"target_va":5369270288,"va":5382947325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f4c0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4213811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e8cdb3bfff","size":5,"soft_fail":false,"target_va":5378733248,"va":5382947054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099ffa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4145564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e864bec0ff","size":5,"soft_fail":false,"target_va":5378801568,"va":5382947127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16901690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e83ae60101","size":5,"soft_fail":false,"target_va":5399848112,"va":5382946417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t193079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e837f20200","size":5,"soft_fail":false,"target_va":5383139568,"va":5382946484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var_os
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1765833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e8c9f11a00","size":5,"soft_fail":false,"target_va":5384712304,"va":5382946466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1779701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e8f5271b00","size":5,"soft_fail":false,"target_va":5384726688,"va":5382946982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::discovery::scan_mods_root
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e83fd2ffff","size":5,"soft_fail":false,"target_va":5382934880,"va":5382946588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::add_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e8f79dffff","size":5,"soft_fail":false,"target_va":5382921568,"va":5382946660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::installed_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-42400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e8605affff","size":5,"soft_fail":false,"target_va":5382903888,"va":5382946283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::minting_user
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-59971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382946240,"raw_hex":"e8bd15ffff","size":5,"soft_fail":false,"target_va":5382886592,"va":5382946558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":52328,"parent_record_offset":52192,"ancestor_record_offset":52192,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":52500,"parent_record_offset":52192,"ancestor_record_offset":52192,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":52444,"parent_record_offset":52192,"ancestor_record_offset":52192,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":53716,"parent_record_offset":52192,"ancestor_record_offset":52192,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::mods_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":53512,"parent_record_offset":52192,"ancestor_record_offset":52192,"inlinee":45514,"source_line":13,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":53964,"parent_record_offset":53940,"ancestor_record_offset":52192,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":53988,"parent_record_offset":53964,"ancestor_record_offset":52192,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":54012,"parent_record_offset":53988,"ancestor_record_offset":52192,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":53940,"parent_record_offset":53884,"ancestor_record_offset":52192,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":53912,"parent_record_offset":53884,"ancestor_record_offset":52192,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":53884,"parent_record_offset":53860,"ancestor_record_offset":52192,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":53860,"parent_record_offset":53836,"ancestor_record_offset":52192,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":54108,"parent_record_offset":54084,"ancestor_record_offset":52192,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":683,"record_offset":53084,"parent_record_offset":53060,"ancestor_record_offset":52192,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":683,"record_offset":52988,"parent_record_offset":52784,"ancestor_record_offset":52192,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":55152,"parent_record_offset":55128,"ancestor_record_offset":52192,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":683,"record_offset":53396,"parent_record_offset":53372,"ancestor_record_offset":52192,"inlinee":11531,"source_line":161,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":683,"record_offset":52468,"parent_record_offset":52444,"ancestor_record_offset":52192,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":683,"record_offset":53348,"parent_record_offset":53324,"ancestor_record_offset":52192,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":53324,"parent_record_offset":53300,"ancestor_record_offset":52192,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":683,"record_offset":52356,"parent_record_offset":52328,"ancestor_record_offset":52192,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":683,"record_offset":52380,"parent_record_offset":52356,"ancestor_record_offset":52192,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":683,"record_offset":52952,"parent_record_offset":52928,"ancestor_record_offset":52192,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":56080,"parent_record_offset":56052,"ancestor_record_offset":52192,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":56108,"parent_record_offset":56080,"ancestor_record_offset":52192,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":54672,"parent_record_offset":54644,"ancestor_record_offset":52192,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":53112,"parent_record_offset":53060,"ancestor_record_offset":52192,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":683,"record_offset":55372,"parent_record_offset":55348,"ancestor_record_offset":52192,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":52696,"parent_record_offset":52660,"ancestor_record_offset":52192,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":53580,"parent_record_offset":53512,"ancestor_record_offset":52192,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":52816,"parent_record_offset":52784,"ancestor_record_offset":52192,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":683,"record_offset":55184,"parent_record_offset":55100,"ancestor_record_offset":52192,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":53448,"parent_record_offset":52528,"ancestor_record_offset":52192,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":52412,"parent_record_offset":52328,"ancestor_record_offset":52192,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":55640,"parent_record_offset":55616,"ancestor_record_offset":52192,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":52904,"parent_record_offset":52784,"ancestor_record_offset":52192,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":55492,"parent_record_offset":55468,"ancestor_record_offset":52192,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":52928,"parent_record_offset":52904,"ancestor_record_offset":52192,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":55316,"parent_record_offset":55288,"ancestor_record_offset":52192,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":55348,"parent_record_offset":55316,"ancestor_record_offset":52192,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":55288,"parent_record_offset":55248,"ancestor_record_offset":52192,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":55128,"parent_record_offset":55100,"ancestor_record_offset":52192,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":53300,"parent_record_offset":53276,"ancestor_record_offset":52192,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":55616,"parent_record_offset":55012,"ancestor_record_offset":52192,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":55468,"parent_record_offset":55248,"ancestor_record_offset":52192,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":55100,"parent_record_offset":55044,"ancestor_record_offset":52192,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":55248,"parent_record_offset":55220,"ancestor_record_offset":52192,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":56020,"parent_record_offset":55988,"ancestor_record_offset":52192,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":55072,"parent_record_offset":55044,"ancestor_record_offset":52192,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":55012,"parent_record_offset":54952,"ancestor_record_offset":52192,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":54952,"parent_record_offset":54928,"ancestor_record_offset":52192,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":55988,"parent_record_offset":54952,"ancestor_record_offset":52192,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":54984,"parent_record_offset":54952,"ancestor_record_offset":52192,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":55044,"parent_record_offset":55012,"ancestor_record_offset":52192,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":56052,"parent_record_offset":56020,"ancestor_record_offset":52192,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":54928,"parent_record_offset":54904,"ancestor_record_offset":52192,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":55220,"parent_record_offset":55012,"ancestor_record_offset":52192,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":683,"record_offset":53276,"parent_record_offset":53252,"ancestor_record_offset":52192,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":53224,"parent_record_offset":52528,"ancestor_record_offset":52192,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":53252,"parent_record_offset":52528,"ancestor_record_offset":52192,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":52528,"parent_record_offset":52500,"ancestor_record_offset":52192,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::env::var_os
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\env.rs
//     evidence={"module":683,"record_offset":53552,"parent_record_offset":53512,"ancestor_record_offset":52192,"inlinee":10543,"source_line":259,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":52604,"parent_record_offset":52576,"ancestor_record_offset":52192,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":683,"record_offset":52864,"parent_record_offset":52840,"ancestor_record_offset":52192,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":683,"record_offset":52724,"parent_record_offset":52696,"ancestor_record_offset":52192,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":53060,"parent_record_offset":52660,"ancestor_record_offset":52192,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":52576,"parent_record_offset":52528,"ancestor_record_offset":52192,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":53672,"parent_record_offset":53648,"ancestor_record_offset":52192,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":683,"record_offset":52752,"parent_record_offset":52724,"ancestor_record_offset":52192,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":683,"record_offset":52784,"parent_record_offset":52752,"ancestor_record_offset":52192,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":683,"record_offset":52660,"parent_record_offset":52632,"ancestor_record_offset":52192,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":683,"record_offset":52632,"parent_record_offset":52604,"ancestor_record_offset":52192,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mods_root::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":53648,"parent_record_offset":53616,"ancestor_record_offset":52192,"inlinee":45516,"source_line":16,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::rediscover_mods::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x174A5
// 首条位置: {"module":2052,"inlinee":95397}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2052,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":5012,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2052,"record_offset":5240,"parent_record_offset":5216,"ancestor_record_offset":5012,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2052,"record_offset":5264,"parent_record_offset":5240,"ancestor_record_offset":5012,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2052,"record_offset":5288,"parent_record_offset":5264,"ancestor_record_offset":5012,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2052,"record_offset":5216,"parent_record_offset":5160,"ancestor_record_offset":5012,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2052,"record_offset":5188,"parent_record_offset":5160,"ancestor_record_offset":5012,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2052,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":5012,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2052,"record_offset":5136,"parent_record_offset":5112,"ancestor_record_offset":5012,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2052,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":5012,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02052.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::scan_mods_root
// 记录形式: compiled_procedure; instances=1
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":25056,"rva":14225760,"va":5382934880}
// 编译实例 RVAs: 0xd91160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1751235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8c3b81a00","size":5,"soft_fail":false,"target_va":5384689616,"va":5382938376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1753893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e825c31a00","size":5,"soft_fail":false,"target_va":5384689312,"va":5382935414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1832981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e815f81b00","size":5,"soft_fail":false,"target_va":5384768304,"va":5382935318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1703552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e880fe1900","size":5,"soft_fail":false,"target_va":5384644912,"va":5382941355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Components as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1835227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8db001c00","size":5,"soft_fail":false,"target_va":5384775360,"va":5382940128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1719309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e80d3c1a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5382936926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::components
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1713038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e88e231a00","size":5,"soft_fail":false,"target_va":5384653152,"va":5382940109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::extension
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1721181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e85d431a00","size":5,"soft_fail":false,"target_va":5384656912,"va":5382935726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1721561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8d9441a00","size":5,"soft_fail":false,"target_va":5384657024,"va":5382935458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_stem
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1721805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8cd451a00","size":5,"soft_fail":false,"target_va":5384657312,"va":5382935502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_dir
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1720961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e881421a00","size":5,"soft_fail":false,"target_va":5384656400,"va":5382935434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1716117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8952f1a00","size":5,"soft_fail":false,"target_va":5384656512,"va":5382940390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1713551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e88f251a00","size":5,"soft_fail":false,"target_va":5384653728,"va":5382940172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1823174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8c6d11b00","size":5,"soft_fail":false,"target_va":5384764464,"va":5382941285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7256973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8734491ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5382935656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=35
//     evidence={"assembly":"\tcallq\t-7258124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8f43f91ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382936823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14213456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8b01e27ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5382935643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13674891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e875562fff","size":5,"soft_fail":false,"target_va":5369270288,"va":5382945174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13610075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8a55330ff","size":5,"soft_fail":false,"target_va":5369335008,"va":5382945078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13596199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8d98930ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5382935042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099f90
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13602770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e82e7030ff","size":5,"soft_fail":false,"target_va":5369339792,"va":5382942557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b05c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13513703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e819cc31ff","size":5,"soft_fail":false,"target_va":5369431488,"va":5382945186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9454915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8bdba6fff","size":5,"soft_fail":false,"target_va":5373480896,"va":5382935806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8838086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e83a2479ff","size":5,"soft_fail":false,"target_va":5374098704,"va":5382936785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4211163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e825bebfff","size":5,"soft_fail":false,"target_va":5378733248,"va":5382944406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c94070
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1037410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e89e2bf0ff","size":5,"soft_fail":false,"target_va":5381898352,"va":5382935757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db7490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t152209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e891520200","size":5,"soft_fail":false,"target_va":5383091344,"va":5382939130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417bb7c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10656211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8d399a200","size":5,"soft_fail":false,"target_va":5393594304,"va":5382938088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da4710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16851295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e85f210101","size":5,"soft_fail":false,"target_va":5399791376,"va":5382940076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da49c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16850615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8b71e0101","size":5,"soft_fail":false,"target_va":5399792064,"va":5382941444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dacfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16884606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e87ea30101","size":5,"soft_fail":false,"target_va":5399826368,"va":5382941757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db0210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16901936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e830e70101","size":5,"soft_fail":false,"target_va":5399839248,"va":5382937307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db0b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16901099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8ebe30101","size":5,"soft_fail":false,"target_va":5399841632,"va":5382940528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db0df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16901807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8afe60101","size":5,"soft_fail":false,"target_va":5399842288,"va":5382940476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db0f30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16901457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e851e50101","size":5,"soft_fail":false,"target_va":5399842608,"va":5382941146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db1020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16906761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e809fa0101","size":5,"soft_fail":false,"target_va":5399842848,"va":5382936082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db1110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16904540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e85cf10101","size":5,"soft_fail":false,"target_va":5399843088,"va":5382938543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db1220
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16904943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8eff20101","size":5,"soft_fail":false,"target_va":5399843360,"va":5382938412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db12e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16903675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8fbed0101","size":5,"soft_fail":false,"target_va":5399843552,"va":5382939872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db13f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16905315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e863f40101","size":5,"soft_fail":false,"target_va":5399843824,"va":5382938504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db1f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16906820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e844fa0101","size":5,"soft_fail":false,"target_va":5399846672,"va":5382939847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33411213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e88dd0fd01","size":5,"soft_fail":false,"target_va":5416350384,"va":5382939166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34121074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e872a50802","size":5,"soft_fail":false,"target_va":5417057040,"va":5382935961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34122031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e82fa90802","size":5,"soft_fail":false,"target_va":5417057056,"va":5382935020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1856827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e83b551c00","size":5,"soft_fail":false,"target_va":5384794864,"va":5382938032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33463870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e83e9efe01","size":5,"soft_fail":false,"target_va":5416401488,"va":5382937613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33679069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8dde60102","size":5,"soft_fail":false,"target_va":5416620794,"va":5382941720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-814910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8c290f3ff","size":5,"soft_fail":false,"target_va":5382122800,"va":5382937705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33605604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8e4c70002","size":5,"soft_fail":false,"target_va":5416546544,"va":5382940935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1685062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e846b61900","size":5,"soft_fail":false,"target_va":5384624080,"va":5382939013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hash::to_hex
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2179608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e818422100","size":5,"soft_fail":false,"target_va":5385116576,"va":5382936963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2179888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e830432100","size":5,"soft_fail":false,"target_va":5385116768,"va":5382936875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$zip..read..ZipFile$GT$
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t1938619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8bb941d00","size":5,"soft_fail":false,"target_va":5384878928,"va":5382940304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$zip..read..zip_archive..ZipArchive$LT$std..io..cursor..Cursor$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16833323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e82bdb0001","size":5,"soft_fail":false,"target_va":5399774192,"va":5382940864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16893430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8f6c50101","size":5,"soft_fail":false,"target_va":5399835232,"va":5382941797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1894593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8c1e81c00","size":5,"soft_fail":false,"target_va":5384830080,"va":5382935482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::standard_app_root_path
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t201891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8a3140300","size":5,"soft_fail":false,"target_va":5383138784,"va":5382936888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::File::create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16906699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8cbf90101","size":5,"soft_fail":false,"target_va":5399847168,"va":5382940464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1815874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e842b51b00","size":5,"soft_fail":false,"target_va":5384751936,"va":5382936057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::copy::stack_buffer_copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16906750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8fef90101","size":5,"soft_fail":false,"target_va":5399847264,"va":5382940509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1786738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e872431b00","size":5,"soft_fail":false,"target_va":5384724912,"va":5382938169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1790191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8ef501b00","size":5,"soft_fail":false,"target_va":5384725152,"va":5382934956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1784581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8053b1b00","size":5,"soft_fail":false,"target_va":5384723232,"va":5382938646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::rename
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1783770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8da371b00","size":5,"soft_fail":false,"target_va":5384724448,"va":5382940673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1784915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8533c1b00","size":5,"soft_fail":false,"target_va":5384726688,"va":5382941768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::discovery::effective_mod_root
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e85c2d0000","size":5,"soft_fail":false,"target_va":5382947808,"va":5382936191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::discovery::touch_last_used
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e805280000","size":5,"soft_fail":false,"target_va":5382947360,"va":5382937110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::identity::folder_mod_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14392643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8439ddb00","size":5,"soft_fail":false,"target_va":5397329024,"va":5382936376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::identity::zip_mod_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14384790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382934880,"raw_hex":"e8967edb00","size":5,"soft_fail":false,"target_va":5397324240,"va":5382939445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*43220621(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5382934880,"raw_hex":"ff158d7e9302","size":6,"soft_fail":false,"va":5382940549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":683,"record_offset":38020,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":25332,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":39408,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":40236,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":683,"record_offset":25388,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":25624,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":26720,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":25304,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":40152,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":26780,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":42952,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":5639,"source_line":149,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":25244,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":25952,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::extracted_zip_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":27832,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":45535,"source_line":384,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::log_partless
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":38676,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":45522,"source_line":364,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::thumbnail_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":39260,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":45525,"source_line":374,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::overlay::ModParts::is_empty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/overlay.rs
//     evidence={"module":683,"record_offset":37992,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":45506,"source_line":29,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::overlay::sort_by_load_order
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/overlay.rs
//     evidence={"module":683,"record_offset":42744,"parent_record_offset":25056,"ancestor_record_offset":25056,"inlinee":45465,"source_line":349,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":26532,"parent_record_offset":26508,"ancestor_record_offset":25056,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":26556,"parent_record_offset":26532,"ancestor_record_offset":25056,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":28644,"parent_record_offset":28620,"ancestor_record_offset":25056,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":28668,"parent_record_offset":28644,"ancestor_record_offset":25056,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":26580,"parent_record_offset":26556,"ancestor_record_offset":25056,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":28692,"parent_record_offset":28668,"ancestor_record_offset":25056,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":26508,"parent_record_offset":26356,"ancestor_record_offset":25056,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":28620,"parent_record_offset":28564,"ancestor_record_offset":25056,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$4::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":683,"record_offset":31344,"parent_record_offset":31320,"ancestor_record_offset":25056,"inlinee":6483,"source_line":349,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":683,"record_offset":29232,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":683,"record_offset":29280,"parent_record_offset":29256,"ancestor_record_offset":25056,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":31476,"parent_record_offset":31448,"ancestor_record_offset":25056,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":26288,"parent_record_offset":26248,"ancestor_record_offset":25056,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":28592,"parent_record_offset":28564,"ancestor_record_offset":25056,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":28564,"parent_record_offset":28540,"ancestor_record_offset":25056,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":31532,"parent_record_offset":31504,"ancestor_record_offset":25056,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":31504,"parent_record_offset":31476,"ancestor_record_offset":25056,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":26356,"parent_record_offset":26320,"ancestor_record_offset":25056,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":26320,"parent_record_offset":26288,"ancestor_record_offset":25056,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":28540,"parent_record_offset":28516,"ancestor_record_offset":25056,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":26396,"parent_record_offset":26356,"ancestor_record_offset":25056,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":42776,"parent_record_offset":42744,"ancestor_record_offset":25056,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":38092,"parent_record_offset":38056,"ancestor_record_offset":25056,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":38128,"parent_record_offset":38092,"ancestor_record_offset":25056,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":26204,"parent_record_offset":26128,"ancestor_record_offset":25056,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":38056,"parent_record_offset":38020,"ancestor_record_offset":25056,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":42808,"parent_record_offset":42776,"ancestor_record_offset":25056,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":683,"record_offset":31592,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":16752,"source_line":268,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::replace_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":683,"record_offset":31616,"parent_record_offset":31592,"ancestor_record_offset":25056,"inlinee":32454,"source_line":708,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":683,"record_offset":31392,"parent_record_offset":31344,"ancestor_record_offset":25056,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":40016,"parent_record_offset":39980,"ancestor_record_offset":25056,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":39980,"parent_record_offset":39956,"ancestor_record_offset":25056,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":41120,"parent_record_offset":41076,"ancestor_record_offset":25056,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":31448,"parent_record_offset":31420,"ancestor_record_offset":25056,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":26176,"parent_record_offset":26128,"ancestor_record_offset":25056,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":31160,"parent_record_offset":31136,"ancestor_record_offset":25056,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":39436,"parent_record_offset":39408,"ancestor_record_offset":25056,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":26248,"parent_record_offset":26204,"ancestor_record_offset":25056,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":26092,"parent_record_offset":26060,"ancestor_record_offset":25056,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":31768,"parent_record_offset":31644,"ancestor_record_offset":25056,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":26128,"parent_record_offset":26092,"ancestor_record_offset":25056,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":683,"record_offset":31792,"parent_record_offset":31768,"ancestor_record_offset":25056,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":683,"record_offset":26060,"parent_record_offset":26024,"ancestor_record_offset":25056,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::array_string::ArrayString::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":683,"record_offset":28284,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":45536,"source_line":411,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::array_string::ArrayString::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":683,"record_offset":28332,"parent_record_offset":28308,"ancestor_record_offset":25056,"inlinee":45537,"source_line":88,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::array_string::impl$2::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":683,"record_offset":28308,"parent_record_offset":28284,"ancestor_record_offset":25056,"inlinee":33669,"source_line":435,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":683,"record_offset":26420,"parent_record_offset":26396,"ancestor_record_offset":25056,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":683,"record_offset":31924,"parent_record_offset":31836,"ancestor_record_offset":25056,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":683,"record_offset":43520,"parent_record_offset":43492,"ancestor_record_offset":25056,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":683,"record_offset":43420,"parent_record_offset":43216,"ancestor_record_offset":25056,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":29120,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":31248,"parent_record_offset":31220,"ancestor_record_offset":25056,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":27484,"parent_record_offset":27456,"ancestor_record_offset":25056,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":683,"record_offset":33888,"parent_record_offset":33864,"ancestor_record_offset":25056,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":43988,"parent_record_offset":43964,"ancestor_record_offset":25056,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":27044,"parent_record_offset":27020,"ancestor_record_offset":25056,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":27128,"parent_record_offset":27104,"ancestor_record_offset":25056,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":683,"record_offset":29316,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":27656,"parent_record_offset":26936,"ancestor_record_offset":25056,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":26468,"parent_record_offset":26444,"ancestor_record_offset":25056,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":683,"record_offset":31740,"parent_record_offset":31644,"ancestor_record_offset":25056,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":683,"record_offset":31220,"parent_record_offset":31192,"ancestor_record_offset":25056,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":683,"record_offset":31192,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":31868,"parent_record_offset":31836,"ancestor_record_offset":25056,"inlinee":8990,"source_line":2809,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":39344,"parent_record_offset":39260,"ancestor_record_offset":25056,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":25412,"parent_record_offset":25388,"ancestor_record_offset":25056,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":25464,"parent_record_offset":25436,"ancestor_record_offset":25056,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":39288,"parent_record_offset":39260,"ancestor_record_offset":25056,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":25436,"parent_record_offset":25412,"ancestor_record_offset":25056,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":683,"record_offset":43384,"parent_record_offset":43360,"ancestor_record_offset":25056,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":26444,"parent_record_offset":26420,"ancestor_record_offset":25056,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":44972,"parent_record_offset":44944,"ancestor_record_offset":25056,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":43548,"parent_record_offset":43492,"ancestor_record_offset":25056,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":683,"record_offset":27276,"parent_record_offset":27248,"ancestor_record_offset":25056,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":683,"record_offset":27248,"parent_record_offset":27220,"ancestor_record_offset":25056,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":683,"record_offset":39368,"parent_record_offset":39344,"ancestor_record_offset":25056,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":25512,"parent_record_offset":25488,"ancestor_record_offset":25056,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":43128,"parent_record_offset":43092,"ancestor_record_offset":25056,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":25488,"parent_record_offset":25464,"ancestor_record_offset":25056,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":29144,"parent_record_offset":29120,"ancestor_record_offset":25056,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":29256,"parent_record_offset":29232,"ancestor_record_offset":25056,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":43248,"parent_record_offset":43216,"ancestor_record_offset":25056,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":683,"record_offset":44020,"parent_record_offset":43936,"ancestor_record_offset":25056,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":683,"record_offset":26636,"parent_record_offset":26204,"ancestor_record_offset":25056,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":26660,"parent_record_offset":26636,"ancestor_record_offset":25056,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=218
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":25568,"parent_record_offset":25536,"ancestor_record_offset":25056,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":31708,"parent_record_offset":31684,"ancestor_record_offset":25056,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":44476,"parent_record_offset":44452,"ancestor_record_offset":25056,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":43336,"parent_record_offset":43216,"ancestor_record_offset":25056,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":27392,"parent_record_offset":27368,"ancestor_record_offset":25056,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":40048,"parent_record_offset":40016,"ancestor_record_offset":25056,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":43360,"parent_record_offset":43336,"ancestor_record_offset":25056,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":39504,"parent_record_offset":39436,"ancestor_record_offset":25056,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":25268,"parent_record_offset":25244,"ancestor_record_offset":25056,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":25536,"parent_record_offset":25512,"ancestor_record_offset":25056,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":28164,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":28200,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":27568,"parent_record_offset":27540,"ancestor_record_offset":25056,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":27600,"parent_record_offset":27568,"ancestor_record_offset":25056,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":683,"record_offset":32096,"parent_record_offset":32068,"ancestor_record_offset":25056,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":683,"record_offset":31644,"parent_record_offset":31616,"ancestor_record_offset":25056,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":683,"record_offset":31684,"parent_record_offset":31644,"ancestor_record_offset":25056,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":683,"record_offset":42840,"parent_record_offset":42808,"ancestor_record_offset":25056,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":683,"record_offset":32012,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":8326,"source_line":1430,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":683,"record_offset":32068,"parent_record_offset":32040,"ancestor_record_offset":25056,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":683,"record_offset":32040,"parent_record_offset":32012,"ancestor_record_offset":25056,"inlinee":22388,"source_line":603,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":683,"record_offset":27512,"parent_record_offset":27484,"ancestor_record_offset":25056,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":29060,"parent_record_offset":29036,"ancestor_record_offset":25056,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":29084,"parent_record_offset":29060,"ancestor_record_offset":25056,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":40076,"parent_record_offset":40016,"ancestor_record_offset":25056,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":40108,"parent_record_offset":39980,"ancestor_record_offset":25056,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":683,"record_offset":25740,"parent_record_offset":25712,"ancestor_record_offset":25056,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":27188,"parent_record_offset":27160,"ancestor_record_offset":25056,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":27220,"parent_record_offset":27188,"ancestor_record_offset":25056,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":27160,"parent_record_offset":26936,"ancestor_record_offset":25056,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":27020,"parent_record_offset":26936,"ancestor_record_offset":25056,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":43964,"parent_record_offset":43936,"ancestor_record_offset":25056,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":27684,"parent_record_offset":26936,"ancestor_record_offset":25056,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":27104,"parent_record_offset":26936,"ancestor_record_offset":25056,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":683,"record_offset":26992,"parent_record_offset":26936,"ancestor_record_offset":25056,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":683,"record_offset":27456,"parent_record_offset":27428,"ancestor_record_offset":25056,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":26828,"parent_record_offset":26804,"ancestor_record_offset":25056,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":27428,"parent_record_offset":27320,"ancestor_record_offset":25056,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":26860,"parent_record_offset":26828,"ancestor_record_offset":25056,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":44452,"parent_record_offset":43848,"ancestor_record_offset":25056,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":27368,"parent_record_offset":27344,"ancestor_record_offset":25056,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":44304,"parent_record_offset":44084,"ancestor_record_offset":25056,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":27776,"parent_record_offset":26936,"ancestor_record_offset":25056,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":43936,"parent_record_offset":43880,"ancestor_record_offset":25056,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":44084,"parent_record_offset":44056,"ancestor_record_offset":25056,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":27344,"parent_record_offset":27320,"ancestor_record_offset":25056,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":26912,"parent_record_offset":26888,"ancestor_record_offset":25056,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":26888,"parent_record_offset":26828,"ancestor_record_offset":25056,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":44884,"parent_record_offset":44852,"ancestor_record_offset":25056,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":43908,"parent_record_offset":43880,"ancestor_record_offset":25056,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":43848,"parent_record_offset":43788,"ancestor_record_offset":25056,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":43788,"parent_record_offset":43764,"ancestor_record_offset":25056,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":26936,"parent_record_offset":26912,"ancestor_record_offset":25056,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":44852,"parent_record_offset":43788,"ancestor_record_offset":25056,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":43820,"parent_record_offset":43788,"ancestor_record_offset":25056,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":43880,"parent_record_offset":43848,"ancestor_record_offset":25056,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":44916,"parent_record_offset":44884,"ancestor_record_offset":25056,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":43764,"parent_record_offset":43740,"ancestor_record_offset":25056,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":44056,"parent_record_offset":43848,"ancestor_record_offset":25056,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":27320,"parent_record_offset":26936,"ancestor_record_offset":25056,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":683,"record_offset":26804,"parent_record_offset":26780,"ancestor_record_offset":25056,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":29036,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":42980,"parent_record_offset":42952,"ancestor_record_offset":25056,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":25684,"parent_record_offset":25656,"ancestor_record_offset":25056,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":25988,"parent_record_offset":25952,"ancestor_record_offset":25056,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":29892,"parent_record_offset":29840,"ancestor_record_offset":25056,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":29864,"parent_record_offset":29840,"ancestor_record_offset":25056,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::create_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":29840,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":29736,"parent_record_offset":29712,"ancestor_record_offset":25056,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":28136,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":30712,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":13843,"source_line":3232,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::rename
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":33864,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":13844,"source_line":2818,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":43036,"parent_record_offset":43008,"ancestor_record_offset":25056,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":683,"record_offset":43296,"parent_record_offset":43272,"ancestor_record_offset":25056,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":683,"record_offset":43156,"parent_record_offset":43128,"ancestor_record_offset":25056,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":43492,"parent_record_offset":43092,"ancestor_record_offset":25056,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":43008,"parent_record_offset":42980,"ancestor_record_offset":25056,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\copy.rs
//     evidence={"module":683,"record_offset":32960,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":25036,"source_line":62,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::generic_copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\copy.rs
//     evidence={"module":683,"record_offset":32984,"parent_record_offset":32960,"ancestor_record_offset":25056,"inlinee":25037,"source_line":77,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::impl$4::copy_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\copy.rs
//     evidence={"module":683,"record_offset":33008,"parent_record_offset":32984,"ancestor_record_offset":25056,"inlinee":33579,"source_line":201,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":683,"record_offset":29924,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":23648,"source_line":98,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":683,"record_offset":33168,"parent_record_offset":33144,"ancestor_record_offset":25056,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":29712,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":14220,"source_line":3443,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":28252,"parent_record_offset":28228,"ancestor_record_offset":25056,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":31972,"parent_record_offset":31948,"ancestor_record_offset":25056,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":683,"record_offset":25712,"parent_record_offset":25684,"ancestor_record_offset":25056,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":683,"record_offset":26024,"parent_record_offset":25988,"ancestor_record_offset":25056,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":683,"record_offset":43184,"parent_record_offset":43156,"ancestor_record_offset":25056,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":683,"record_offset":43216,"parent_record_offset":43184,"ancestor_record_offset":25056,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":683,"record_offset":43092,"parent_record_offset":43064,"ancestor_record_offset":25056,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":683,"record_offset":43064,"parent_record_offset":43036,"ancestor_record_offset":25056,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mod_cache_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":28228,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":45527,"source_line":21,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::sanitized_zip_entry_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":31836,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":45542,"source_line":491,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::sanitized_zip_entry_path::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":31948,"parent_record_offset":31924,"ancestor_record_offset":25056,"inlinee":45544,"source_line":495,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::scan_mods_root::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":25656,"parent_record_offset":25624,"ancestor_record_offset":25056,"inlinee":45532,"source_line":243,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::scan_mods_root::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":25820,"parent_record_offset":25788,"ancestor_record_offset":25056,"inlinee":45533,"source_line":245,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::scan_mods_root::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":26748,"parent_record_offset":26720,"ancestor_record_offset":25056,"inlinee":45534,"source_line":263,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] zip::read::ZipFile::name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":683,"record_offset":31320,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":45541,"source_line":858,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] zip::read::zip_archive::ZipArchive::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":683,"record_offset":31136,"parent_record_offset":27832,"ancestor_record_offset":25056,"inlinee":45539,"source_line":483,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::scan_mods_root::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1DC
// 首条位置: {"module":683,"inlinee":45532}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":25684,"parent_record_offset":25656,"ancestor_record_offset":25656,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":683,"record_offset":25740,"parent_record_offset":25712,"ancestor_record_offset":25656,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":683,"record_offset":25712,"parent_record_offset":25684,"ancestor_record_offset":25656,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::scan_mods_root::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 245
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1DD
// 首条位置: {"module":683,"inlinee":45533}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":25848,"parent_record_offset":25820,"ancestor_record_offset":25820,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":683,"record_offset":25904,"parent_record_offset":25876,"ancestor_record_offset":25820,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":683,"record_offset":25876,"parent_record_offset":25848,"ancestor_record_offset":25820,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::scan_mods_root::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 263
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1DE
// 首条位置: {"module":683,"inlinee":45534}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::scan_mods_root::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 307
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1743C
// 首条位置: {"module":2029,"inlinee":95292}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2029,"record_offset":4280,"parent_record_offset":4076,"ancestor_record_offset":4076,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2029,"record_offset":4336,"parent_record_offset":4076,"ancestor_record_offset":4076,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2029,"record_offset":4196,"parent_record_offset":4076,"ancestor_record_offset":4076,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":2029,"record_offset":4140,"parent_record_offset":4076,"ancestor_record_offset":4076,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::identity::impl$5::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":2029,"record_offset":4112,"parent_record_offset":4076,"ancestor_record_offset":4076,"inlinee":29159,"source_line":47,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2029,"record_offset":4304,"parent_record_offset":4280,"ancestor_record_offset":4076,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2029,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":4076,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2029,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":4076,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":2029,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":4076,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2029,"record_offset":4384,"parent_record_offset":4360,"ancestor_record_offset":4076,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2029,"record_offset":4360,"parent_record_offset":4336,"ancestor_record_offset":4076,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2029,"record_offset":4408,"parent_record_offset":4384,"ancestor_record_offset":4076,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::mod_at_root
// 记录形式: compiled_procedure; instances=1
// 源码行: 331
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":5192,"rva":14218336,"va":5382927456}
// 编译实例 RVAs: 0xd8f460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_string_lossy
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1863911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e8e7701c00","size":5,"soft_fail":false,"target_va":5384792224,"va":5382928308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1728691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e8b3601a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5382927544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1728746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e8ea601a00","size":5,"soft_fail":false,"target_va":5384657024,"va":5382928273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1681627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e8dba81900","size":5,"soft_fail":false,"target_va":5384609408,"va":5382927776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7249737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e8b76091ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5382928420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-7248901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e8fb6391ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382927600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14206220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e8f43a27ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5382928407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4ab10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14399031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e837b6db00","size":5,"soft_fail":false,"target_va":5397326608,"va":5382927572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db0210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16910666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e84a090201","size":5,"soft_fail":false,"target_va":5399839248,"va":5382928577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34128600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e8d8c20802","size":5,"soft_fail":false,"target_va":5417057056,"va":5382928451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1866932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e8b47c1c00","size":5,"soft_fail":false,"target_va":5384794864,"va":5382927927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33692427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e80b1b0202","size":5,"soft_fail":false,"target_va":5416620794,"va":5382928362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::discovery::mod_parts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t21614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382927456,"raw_hex":"e86e540000","size":5,"soft_fail":false,"target_va":5382949456,"va":5382927837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":683,"record_offset":6396,"parent_record_offset":5192,"ancestor_record_offset":5192,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":6200,"parent_record_offset":5192,"ancestor_record_offset":5192,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":683,"record_offset":6480,"parent_record_offset":5192,"ancestor_record_offset":5192,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":7472,"parent_record_offset":5192,"ancestor_record_offset":5192,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":6788,"parent_record_offset":5192,"ancestor_record_offset":5192,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":6256,"parent_record_offset":5192,"ancestor_record_offset":5192,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":6116,"parent_record_offset":5192,"ancestor_record_offset":5192,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::log_partless
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":6508,"parent_record_offset":5192,"ancestor_record_offset":5192,"inlinee":45522,"source_line":364,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::thumbnail_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":8228,"parent_record_offset":5192,"ancestor_record_offset":5192,"inlinee":45525,"source_line":374,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::identity::read_mod_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":683,"record_offset":5348,"parent_record_offset":5192,"ancestor_record_offset":5192,"inlinee":45521,"source_line":226,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::overlay::ModParts::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/overlay.rs
//     evidence={"module":683,"record_offset":6368,"parent_record_offset":5192,"ancestor_record_offset":5192,"inlinee":45506,"source_line":29,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":8036,"parent_record_offset":8012,"ancestor_record_offset":5192,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":8060,"parent_record_offset":8036,"ancestor_record_offset":5192,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":5192,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":5192,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":5192,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":5192,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":8012,"parent_record_offset":7860,"ancestor_record_offset":5192,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":5600,"parent_record_offset":5548,"ancestor_record_offset":5192,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":683,"record_offset":7592,"parent_record_offset":7508,"ancestor_record_offset":5192,"inlinee":5422,"source_line":331,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":683,"record_offset":6444,"parent_record_offset":6420,"ancestor_record_offset":5192,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":7796,"parent_record_offset":7768,"ancestor_record_offset":5192,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":5572,"parent_record_offset":5548,"ancestor_record_offset":5192,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":5548,"parent_record_offset":5524,"ancestor_record_offset":5192,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":7860,"parent_record_offset":7824,"ancestor_record_offset":5192,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":7824,"parent_record_offset":7796,"ancestor_record_offset":5192,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":5524,"parent_record_offset":5504,"ancestor_record_offset":5192,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":7900,"parent_record_offset":7860,"ancestor_record_offset":5192,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":7680,"parent_record_offset":7652,"ancestor_record_offset":5192,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":7708,"parent_record_offset":7680,"ancestor_record_offset":5192,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":7736,"parent_record_offset":7708,"ancestor_record_offset":5192,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":7652,"parent_record_offset":7624,"ancestor_record_offset":5192,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":683,"record_offset":7624,"parent_record_offset":7592,"ancestor_record_offset":5192,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":7768,"parent_record_offset":7736,"ancestor_record_offset":5192,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":683,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":5192,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":6616,"parent_record_offset":6508,"ancestor_record_offset":5192,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":7972,"parent_record_offset":7948,"ancestor_record_offset":5192,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":683,"record_offset":8280,"parent_record_offset":8256,"ancestor_record_offset":5192,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":8312,"parent_record_offset":8228,"ancestor_record_offset":5192,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":8256,"parent_record_offset":8228,"ancestor_record_offset":5192,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":7948,"parent_record_offset":7924,"ancestor_record_offset":5192,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":683,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":5192,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":5192,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":5192,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":683,"record_offset":8140,"parent_record_offset":7736,"ancestor_record_offset":5192,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":5192,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":5404,"parent_record_offset":5348,"ancestor_record_offset":5192,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":6140,"parent_record_offset":6116,"ancestor_record_offset":5192,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":6164,"parent_record_offset":6140,"ancestor_record_offset":5192,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":5192,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":6672,"parent_record_offset":6508,"ancestor_record_offset":5192,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":5192,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":6328,"parent_record_offset":6304,"ancestor_record_offset":5192,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":5192,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":7536,"parent_record_offset":7508,"ancestor_record_offset":5192,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":5376,"parent_record_offset":5348,"ancestor_record_offset":5192,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":683,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":5192,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::mod_at_root::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":7508,"parent_record_offset":7472,"ancestor_record_offset":5192,"inlinee":45524,"source_line":346,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::mod_at_root::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 346
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1D4
// 首条位置: {"module":683,"inlinee":45524}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":683,"record_offset":7592,"parent_record_offset":7508,"ancestor_record_offset":7508,"inlinee":5422,"source_line":331,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":683,"record_offset":7536,"parent_record_offset":7508,"ancestor_record_offset":7508,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":8036,"parent_record_offset":8012,"ancestor_record_offset":7508,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":8060,"parent_record_offset":8036,"ancestor_record_offset":7508,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":7508,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":8012,"parent_record_offset":7860,"ancestor_record_offset":7508,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":7796,"parent_record_offset":7768,"ancestor_record_offset":7508,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":7860,"parent_record_offset":7824,"ancestor_record_offset":7508,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":7824,"parent_record_offset":7796,"ancestor_record_offset":7508,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":7900,"parent_record_offset":7860,"ancestor_record_offset":7508,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":7680,"parent_record_offset":7652,"ancestor_record_offset":7508,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":7708,"parent_record_offset":7680,"ancestor_record_offset":7508,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":7736,"parent_record_offset":7708,"ancestor_record_offset":7508,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":683,"record_offset":7652,"parent_record_offset":7624,"ancestor_record_offset":7508,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":683,"record_offset":7624,"parent_record_offset":7592,"ancestor_record_offset":7508,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":7768,"parent_record_offset":7736,"ancestor_record_offset":7508,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":683,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":7508,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":7972,"parent_record_offset":7948,"ancestor_record_offset":7508,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":7948,"parent_record_offset":7924,"ancestor_record_offset":7508,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":683,"record_offset":8140,"parent_record_offset":7736,"ancestor_record_offset":7508,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":7508,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":683,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":7508,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::log_partless
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 364
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1D2
// 首条位置: {"module":683,"inlinee":45522}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":6616,"parent_record_offset":6508,"ancestor_record_offset":6508,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":6672,"parent_record_offset":6508,"ancestor_record_offset":6508,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":6508,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":6508,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":6556,"parent_record_offset":6532,"ancestor_record_offset":6508,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":6508,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":6508,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":6508,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":6744,"parent_record_offset":6720,"ancestor_record_offset":6508,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::thumbnail_path
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 374
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1D5
// 首条位置: {"module":683,"inlinee":45525}
// 编译实例 RVAs: 0xd939d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141db0210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16893899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945232,"raw_hex":"e8cbc70101","size":5,"soft_fail":false,"target_va":5399839248,"va":5382945344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":8312,"parent_record_offset":8228,"ancestor_record_offset":8228,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":8256,"parent_record_offset":8228,"ancestor_record_offset":8228,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":683,"record_offset":8280,"parent_record_offset":8256,"ancestor_record_offset":8228,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":683,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":8228,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::thumbnail_path::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 377
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17489
// 首条位置: {"module":2045,"inlinee":95369}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":2045,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":7368,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::thumbnail_path::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 378
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1748A
// 首条位置: {"module":2045,"inlinee":95370}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::extracted_zip_root
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 384
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1DF
// 首条位置: {"module":683,"inlinee":45535}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":683,"record_offset":29232,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":683,"record_offset":31592,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":16752,"source_line":268,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::array_string::ArrayString::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":683,"record_offset":28284,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":45536,"source_line":411,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":29120,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":683,"record_offset":29316,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":683,"record_offset":31192,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":28396,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":28164,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":28200,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":683,"record_offset":32012,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":8326,"source_line":1430,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":29036,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":29840,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":28136,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":30712,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":13843,"source_line":3232,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::rename
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":33864,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":13844,"source_line":2818,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::io::copy::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\copy.rs
//     evidence={"module":683,"record_offset":32960,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":25036,"source_line":62,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":683,"record_offset":29924,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":23648,"source_line":98,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":29712,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":14220,"source_line":3443,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":28368,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::mod_cache_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":28228,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":45527,"source_line":21,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::sanitized_zip_entry_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":31836,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":45542,"source_line":491,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] zip::read::ZipFile::name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":683,"record_offset":31320,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":45541,"source_line":858,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] zip::read::zip_archive::ZipArchive::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":683,"record_offset":31136,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":45539,"source_line":483,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":28644,"parent_record_offset":28620,"ancestor_record_offset":27832,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":28668,"parent_record_offset":28644,"ancestor_record_offset":27832,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":28692,"parent_record_offset":28668,"ancestor_record_offset":27832,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":28620,"parent_record_offset":28564,"ancestor_record_offset":27832,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$4::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":683,"record_offset":31344,"parent_record_offset":31320,"ancestor_record_offset":27832,"inlinee":6483,"source_line":349,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":683,"record_offset":29280,"parent_record_offset":29256,"ancestor_record_offset":27832,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":31476,"parent_record_offset":31448,"ancestor_record_offset":27832,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":28592,"parent_record_offset":28564,"ancestor_record_offset":27832,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":28564,"parent_record_offset":28540,"ancestor_record_offset":27832,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":31532,"parent_record_offset":31504,"ancestor_record_offset":27832,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":31504,"parent_record_offset":31476,"ancestor_record_offset":27832,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":28540,"parent_record_offset":28516,"ancestor_record_offset":27832,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::replace_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":683,"record_offset":31616,"parent_record_offset":31592,"ancestor_record_offset":27832,"inlinee":32454,"source_line":708,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":683,"record_offset":31392,"parent_record_offset":31344,"ancestor_record_offset":27832,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":31448,"parent_record_offset":31420,"ancestor_record_offset":27832,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":31420,"parent_record_offset":31392,"ancestor_record_offset":27832,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":31160,"parent_record_offset":31136,"ancestor_record_offset":27832,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":31768,"parent_record_offset":31644,"ancestor_record_offset":27832,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":683,"record_offset":31792,"parent_record_offset":31768,"ancestor_record_offset":27832,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::array_string::ArrayString::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":683,"record_offset":28332,"parent_record_offset":28308,"ancestor_record_offset":27832,"inlinee":45537,"source_line":88,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::array_string::impl$2::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":683,"record_offset":28308,"parent_record_offset":28284,"ancestor_record_offset":27832,"inlinee":33669,"source_line":435,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":683,"record_offset":31924,"parent_record_offset":31836,"ancestor_record_offset":27832,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":31248,"parent_record_offset":31220,"ancestor_record_offset":27832,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":32124,"parent_record_offset":32096,"ancestor_record_offset":27832,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":683,"record_offset":33888,"parent_record_offset":33864,"ancestor_record_offset":27832,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":683,"record_offset":31740,"parent_record_offset":31644,"ancestor_record_offset":27832,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":683,"record_offset":31220,"parent_record_offset":31192,"ancestor_record_offset":27832,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":31868,"parent_record_offset":31836,"ancestor_record_offset":27832,"inlinee":8990,"source_line":2809,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":31892,"parent_record_offset":31868,"ancestor_record_offset":27832,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":29144,"parent_record_offset":29120,"ancestor_record_offset":27832,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":29256,"parent_record_offset":29232,"ancestor_record_offset":27832,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=95
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":28420,"parent_record_offset":28396,"ancestor_record_offset":27832,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":31708,"parent_record_offset":31684,"ancestor_record_offset":27832,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":29764,"parent_record_offset":29736,"ancestor_record_offset":27832,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":32148,"parent_record_offset":32124,"ancestor_record_offset":27832,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":32172,"parent_record_offset":32148,"ancestor_record_offset":27832,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":683,"record_offset":32096,"parent_record_offset":32068,"ancestor_record_offset":27832,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":683,"record_offset":31644,"parent_record_offset":31616,"ancestor_record_offset":27832,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":683,"record_offset":31684,"parent_record_offset":31644,"ancestor_record_offset":27832,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":683,"record_offset":32068,"parent_record_offset":32040,"ancestor_record_offset":27832,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":683,"record_offset":32040,"parent_record_offset":32012,"ancestor_record_offset":27832,"inlinee":22388,"source_line":603,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":29060,"parent_record_offset":29036,"ancestor_record_offset":27832,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":29084,"parent_record_offset":29060,"ancestor_record_offset":27832,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":29892,"parent_record_offset":29840,"ancestor_record_offset":27832,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":29864,"parent_record_offset":29840,"ancestor_record_offset":27832,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":29736,"parent_record_offset":29712,"ancestor_record_offset":27832,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::generic_copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\copy.rs
//     evidence={"module":683,"record_offset":32984,"parent_record_offset":32960,"ancestor_record_offset":27832,"inlinee":25037,"source_line":77,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::impl$4::copy_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\copy.rs
//     evidence={"module":683,"record_offset":33008,"parent_record_offset":32984,"ancestor_record_offset":27832,"inlinee":33579,"source_line":201,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":683,"record_offset":33168,"parent_record_offset":33144,"ancestor_record_offset":27832,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":28252,"parent_record_offset":28228,"ancestor_record_offset":27832,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":31972,"parent_record_offset":31948,"ancestor_record_offset":27832,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::sanitized_zip_entry_path::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":31948,"parent_record_offset":31924,"ancestor_record_offset":27832,"inlinee":45544,"source_line":495,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::extracted_zip_root::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 422
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17497
// 首条位置: {"module":2046,"inlinee":95383}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2046,"record_offset":2204,"parent_record_offset":2184,"ancestor_record_offset":2184,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2046,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":2184,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02046.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2046,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":2184,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::extracted_zip_root::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 423
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17495
// 首条位置: {"module":2046,"inlinee":95381}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2046,"record_offset":800,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2046,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":780,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02046.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2046,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":780,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::trim_orphaned_zip_sidecars
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 444
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1DA
// 首条位置: {"module":683,"inlinee":45530}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":20156,"parent_record_offset":17616,"ancestor_record_offset":17616,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":17836,"parent_record_offset":17616,"ancestor_record_offset":17616,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":17808,"parent_record_offset":17616,"ancestor_record_offset":17616,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":683,"record_offset":18784,"parent_record_offset":17616,"ancestor_record_offset":17616,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":683,"record_offset":19960,"parent_record_offset":17616,"ancestor_record_offset":17616,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":20212,"parent_record_offset":17616,"ancestor_record_offset":17616,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":20072,"parent_record_offset":17616,"ancestor_record_offset":17616,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":18868,"parent_record_offset":17616,"ancestor_record_offset":17616,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":17744,"parent_record_offset":17616,"ancestor_record_offset":17616,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":20324,"parent_record_offset":17616,"ancestor_record_offset":17616,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":20016,"parent_record_offset":17616,"ancestor_record_offset":17616,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":20880,"parent_record_offset":20856,"ancestor_record_offset":17616,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":20904,"parent_record_offset":20880,"ancestor_record_offset":17616,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":20928,"parent_record_offset":20904,"ancestor_record_offset":17616,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":20856,"parent_record_offset":20800,"ancestor_record_offset":17616,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":20828,"parent_record_offset":20800,"ancestor_record_offset":17616,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":20800,"parent_record_offset":20776,"ancestor_record_offset":17616,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":20776,"parent_record_offset":20752,"ancestor_record_offset":17616,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":17860,"parent_record_offset":17836,"ancestor_record_offset":17616,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":21544,"parent_record_offset":21520,"ancestor_record_offset":17616,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":683,"record_offset":18496,"parent_record_offset":18468,"ancestor_record_offset":17616,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":683,"record_offset":18396,"parent_record_offset":18192,"ancestor_record_offset":17616,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":19532,"parent_record_offset":19508,"ancestor_record_offset":17616,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":683,"record_offset":20040,"parent_record_offset":20016,"ancestor_record_offset":17616,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":19100,"parent_record_offset":19076,"ancestor_record_offset":17616,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":683,"record_offset":19184,"parent_record_offset":19160,"ancestor_record_offset":17616,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":683,"record_offset":19700,"parent_record_offset":19020,"ancestor_record_offset":17616,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":18736,"parent_record_offset":18712,"ancestor_record_offset":17616,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":683,"record_offset":18360,"parent_record_offset":18336,"ancestor_record_offset":17616,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":21268,"parent_record_offset":21240,"ancestor_record_offset":17616,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":21296,"parent_record_offset":21268,"ancestor_record_offset":17616,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":22012,"parent_record_offset":21984,"ancestor_record_offset":17616,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":18524,"parent_record_offset":18468,"ancestor_record_offset":17616,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":683,"record_offset":19332,"parent_record_offset":19304,"ancestor_record_offset":17616,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":683,"record_offset":19304,"parent_record_offset":19276,"ancestor_record_offset":17616,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":18104,"parent_record_offset":18068,"ancestor_record_offset":17616,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":20180,"parent_record_offset":20156,"ancestor_record_offset":17616,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":18224,"parent_record_offset":18192,"ancestor_record_offset":17616,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":19124,"parent_record_offset":19100,"ancestor_record_offset":17616,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":20656,"parent_record_offset":20632,"ancestor_record_offset":17616,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":18832,"parent_record_offset":18808,"ancestor_record_offset":17616,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":18312,"parent_record_offset":18192,"ancestor_record_offset":17616,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":683,"record_offset":19448,"parent_record_offset":19424,"ancestor_record_offset":17616,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":19984,"parent_record_offset":19960,"ancestor_record_offset":17616,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":18336,"parent_record_offset":18312,"ancestor_record_offset":17616,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":17768,"parent_record_offset":17744,"ancestor_record_offset":17616,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":19616,"parent_record_offset":19588,"ancestor_record_offset":17616,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":683,"record_offset":19644,"parent_record_offset":19616,"ancestor_record_offset":17616,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":683,"record_offset":18808,"parent_record_offset":18784,"ancestor_record_offset":17616,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":683,"record_offset":19560,"parent_record_offset":19532,"ancestor_record_offset":17616,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":20096,"parent_record_offset":20072,"ancestor_record_offset":17616,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":20120,"parent_record_offset":20096,"ancestor_record_offset":17616,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":19244,"parent_record_offset":19216,"ancestor_record_offset":17616,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":19276,"parent_record_offset":19244,"ancestor_record_offset":17616,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":683,"record_offset":19216,"parent_record_offset":19020,"ancestor_record_offset":17616,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":19076,"parent_record_offset":19020,"ancestor_record_offset":17616,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":19728,"parent_record_offset":19020,"ancestor_record_offset":17616,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":683,"record_offset":19160,"parent_record_offset":19020,"ancestor_record_offset":17616,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":683,"record_offset":19852,"parent_record_offset":19020,"ancestor_record_offset":17616,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":683,"record_offset":19508,"parent_record_offset":19484,"ancestor_record_offset":17616,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":18924,"parent_record_offset":18896,"ancestor_record_offset":17616,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":19484,"parent_record_offset":19376,"ancestor_record_offset":17616,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":18712,"parent_record_offset":18688,"ancestor_record_offset":17616,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":683,"record_offset":19920,"parent_record_offset":18924,"ancestor_record_offset":17616,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":19424,"parent_record_offset":19400,"ancestor_record_offset":17616,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":19820,"parent_record_offset":19020,"ancestor_record_offset":17616,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":19400,"parent_record_offset":19376,"ancestor_record_offset":17616,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":18992,"parent_record_offset":18964,"ancestor_record_offset":17616,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":18964,"parent_record_offset":18924,"ancestor_record_offset":17616,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":21208,"parent_record_offset":21176,"ancestor_record_offset":17616,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":21120,"parent_record_offset":21096,"ancestor_record_offset":17616,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":19020,"parent_record_offset":18992,"ancestor_record_offset":17616,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":21176,"parent_record_offset":21120,"ancestor_record_offset":17616,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":21148,"parent_record_offset":21120,"ancestor_record_offset":17616,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":19880,"parent_record_offset":19020,"ancestor_record_offset":17616,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":21240,"parent_record_offset":21208,"ancestor_record_offset":17616,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":21096,"parent_record_offset":21072,"ancestor_record_offset":17616,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":683,"record_offset":19376,"parent_record_offset":19020,"ancestor_record_offset":17616,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":683,"record_offset":18896,"parent_record_offset":18868,"ancestor_record_offset":17616,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":683,"record_offset":18688,"parent_record_offset":18664,"ancestor_record_offset":17616,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":20260,"parent_record_offset":20236,"ancestor_record_offset":17616,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":20236,"parent_record_offset":20212,"ancestor_record_offset":17616,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":20284,"parent_record_offset":20260,"ancestor_record_offset":17616,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":18636,"parent_record_offset":17932,"ancestor_record_offset":17616,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":18664,"parent_record_offset":17932,"ancestor_record_offset":17616,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":683,"record_offset":17932,"parent_record_offset":17892,"ancestor_record_offset":17616,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":18012,"parent_record_offset":17984,"ancestor_record_offset":17616,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":683,"record_offset":18272,"parent_record_offset":18248,"ancestor_record_offset":17616,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":683,"record_offset":18132,"parent_record_offset":18104,"ancestor_record_offset":17616,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":18468,"parent_record_offset":18068,"ancestor_record_offset":17616,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":683,"record_offset":17984,"parent_record_offset":17932,"ancestor_record_offset":17616,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":683,"record_offset":18160,"parent_record_offset":18132,"ancestor_record_offset":17616,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":683,"record_offset":18192,"parent_record_offset":18160,"ancestor_record_offset":17616,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":683,"record_offset":18068,"parent_record_offset":18040,"ancestor_record_offset":17616,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":683,"record_offset":18040,"parent_record_offset":18012,"ancestor_record_offset":17616,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::trim_orphaned_zip_sidecars::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 450
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17441
// 首条位置: {"module":2029,"inlinee":95297}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::is_ok_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2029,"record_offset":31480,"parent_record_offset":31360,"ancestor_record_offset":31360,"inlinee":36640,"source_line":619,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_direct] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":2029,"record_offset":31384,"parent_record_offset":31360,"ancestor_record_offset":31360,"inlinee":13254,"source_line":2610,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_file
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":2029,"record_offset":31528,"parent_record_offset":31504,"ancestor_record_offset":31360,"inlinee":33659,"source_line":2425,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::DirEntry::file_type
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":2029,"record_offset":31408,"parent_record_offset":31384,"ancestor_record_offset":31360,"inlinee":13256,"source_line":181,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::is_file
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":2029,"record_offset":31552,"parent_record_offset":31528,"ancestor_record_offset":31360,"inlinee":33660,"source_line":1199,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":2029,"record_offset":31440,"parent_record_offset":31408,"ancestor_record_offset":31360,"inlinee":13258,"source_line":1187,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::trim_orphaned_zip_sidecars::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":2029,"record_offset":31504,"parent_record_offset":31480,"ancestor_record_offset":31360,"inlinee":95299,"source_line":450,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::trim_orphaned_zip_sidecars::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 450
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17443
// 首条位置: {"module":2029,"inlinee":95299}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::fs::FileType::is_file
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":2029,"record_offset":31528,"parent_record_offset":31504,"ancestor_record_offset":31504,"inlinee":33659,"source_line":2425,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::is_file
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":2029,"record_offset":31552,"parent_record_offset":31528,"ancestor_record_offset":31504,"inlinee":33660,"source_line":1199,"archive":"_pdb/C/modules/02029.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::trim_orphaned_zip_sidecars::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 451
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F7D5
// 首条位置: {"module":3559,"inlinee":128981}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3559,"record_offset":2828,"parent_record_offset":2312,"ancestor_record_offset":2312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3559,"record_offset":2424,"parent_record_offset":2312,"ancestor_record_offset":2312,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsString::into_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":3559,"record_offset":2340,"parent_record_offset":2312,"ancestor_record_offset":2312,"inlinee":22535,"source_line":242,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3559,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":2312,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3559,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":2312,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3559,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":2312,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3559,"record_offset":2680,"parent_record_offset":2624,"ancestor_record_offset":2312,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3559,"record_offset":2652,"parent_record_offset":2624,"ancestor_record_offset":2312,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3559,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":2312,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3559,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":2312,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3559,"record_offset":2936,"parent_record_offset":2900,"ancestor_record_offset":2312,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3559,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2312,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3559,"record_offset":2456,"parent_record_offset":2424,"ancestor_record_offset":2312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3559,"record_offset":2968,"parent_record_offset":2936,"ancestor_record_offset":2312,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3559,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":2312,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3559,"record_offset":2996,"parent_record_offset":2936,"ancestor_record_offset":2312,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3559,"record_offset":3028,"parent_record_offset":2900,"ancestor_record_offset":2312,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Buf::into_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":3559,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":2312,"inlinee":22536,"source_line":92,"archive":"_pdb/C/modules/03559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::trim_orphaned_zip_sidecars::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 456
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1747E
// 首条位置: {"module":2045,"inlinee":95358}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":2045,"record_offset":1836,"parent_record_offset":1724,"ancestor_record_offset":1724,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2045,"record_offset":1784,"parent_record_offset":1724,"ancestor_record_offset":1724,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":2045,"record_offset":1756,"parent_record_offset":1724,"ancestor_record_offset":1724,"inlinee":11098,"source_line":2383,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":2045,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":1724,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":2045,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":1724,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":2045,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":1724,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::trim_orphaned_zip_sidecars::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":2045,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":1724,"inlinee":95360,"source_line":459,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::trim_orphaned_zip_sidecars::closure$2::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":2045,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":1724,"inlinee":95361,"source_line":460,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::trim_orphaned_zip_sidecars::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 459
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17480
// 首条位置: {"module":2045,"inlinee":95360}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::trim_orphaned_zip_sidecars::closure$2::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 460
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17481
// 首条位置: {"module":2045,"inlinee":95361}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":2045,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":1860,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":2045,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":1860,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":2045,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":1860,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::touch_last_used
// 记录形式: compiled_procedure; instances=1
// 源码行: 483
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":56704,"rva":14238240,"va":5382947360}
// 编译实例 RVAs: 0xd94220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1708816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947360,"raw_hex":"e810131a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5382947419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1661739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947360,"raw_hex":"e82b5b1900","size":5,"soft_fail":false,"target_va":5384609408,"va":5382947664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7268774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947360,"raw_hex":"e85a1691ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382947473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13608831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947360,"raw_hex":"e8815830ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5382947674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1805014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947360,"raw_hex":"e8d68a1b00","size":5,"soft_fail":false,"target_va":5384752464,"va":5382947445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":57700,"parent_record_offset":56704,"ancestor_record_offset":56704,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":57756,"parent_record_offset":56704,"ancestor_record_offset":56704,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":57616,"parent_record_offset":56704,"ancestor_record_offset":56704,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":56868,"parent_record_offset":56704,"ancestor_record_offset":56704,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":56840,"parent_record_offset":56704,"ancestor_record_offset":56704,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":57124,"parent_record_offset":57100,"ancestor_record_offset":56704,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":57148,"parent_record_offset":57124,"ancestor_record_offset":56704,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":57172,"parent_record_offset":57148,"ancestor_record_offset":56704,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":57100,"parent_record_offset":57048,"ancestor_record_offset":56704,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":57072,"parent_record_offset":57048,"ancestor_record_offset":56704,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":57048,"parent_record_offset":57024,"ancestor_record_offset":56704,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":57024,"parent_record_offset":57004,"ancestor_record_offset":56704,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":57724,"parent_record_offset":57700,"ancestor_record_offset":56704,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":56904,"parent_record_offset":56868,"ancestor_record_offset":56704,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":57640,"parent_record_offset":57616,"ancestor_record_offset":56704,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":57664,"parent_record_offset":57640,"ancestor_record_offset":56704,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":57804,"parent_record_offset":57780,"ancestor_record_offset":56704,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":683,"record_offset":57780,"parent_record_offset":57756,"ancestor_record_offset":56704,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":57828,"parent_record_offset":57804,"ancestor_record_offset":56704,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::sanitized_zip_entry_path
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 491
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1E6
// 首条位置: {"module":683,"inlinee":45542}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":683,"record_offset":31924,"parent_record_offset":31836,"ancestor_record_offset":31836,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":31868,"parent_record_offset":31836,"ancestor_record_offset":31836,"inlinee":8990,"source_line":2809,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":31892,"parent_record_offset":31868,"ancestor_record_offset":31836,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":31972,"parent_record_offset":31948,"ancestor_record_offset":31836,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::sanitized_zip_entry_path::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":31948,"parent_record_offset":31924,"ancestor_record_offset":31836,"inlinee":45544,"source_line":495,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::sanitized_zip_entry_path::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 495
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1E8
// 首条位置: {"module":683,"inlinee":45544}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":31972,"parent_record_offset":31948,"ancestor_record_offset":31948,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::effective_mod_root
// 记录形式: compiled_procedure; instances=1
// 源码行: 501
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":683,"record_offset":57872,"rva":14238688,"va":5382947808}
// 编译实例 RVAs: 0xd943e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-7269684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947808,"raw_hex":"e8cc1291ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382948383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13678468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947808,"raw_hex":"e87c482fff","size":5,"soft_fail":false,"target_va":5369270288,"va":5382948751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13609242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947808,"raw_hex":"e8e65630ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5382948085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da56b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16847242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947808,"raw_hex":"e88a110101","size":5,"soft_fail":false,"target_va":5399795376,"va":5382948129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: discovery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34108982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947808,"raw_hex":"e836760802","size":5,"soft_fail":false,"target_va":5417057056,"va":5382948069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33672967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947808,"raw_hex":"e807cf0102","size":5,"soft_fail":false,"target_va":5416621600,"va":5382948628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1777190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947808,"raw_hex":"e8261e1b00","size":5,"soft_fail":false,"target_va":5384725152,"va":5382947957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::discovery::mod_parts
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382947808,"raw_hex":"e823060000","size":5,"soft_fail":false,"target_va":5382949456,"va":5382947880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":58020,"parent_record_offset":57872,"ancestor_record_offset":57872,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":58956,"parent_record_offset":57872,"ancestor_record_offset":57872,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":58144,"parent_record_offset":57872,"ancestor_record_offset":57872,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":59120,"parent_record_offset":57872,"ancestor_record_offset":57872,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":58116,"parent_record_offset":57872,"ancestor_record_offset":57872,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":58048,"parent_record_offset":57872,"ancestor_record_offset":57872,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::overlay::ModParts::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/overlay.rs
//     evidence={"module":683,"record_offset":58928,"parent_record_offset":57872,"ancestor_record_offset":57872,"inlinee":45506,"source_line":29,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":58536,"parent_record_offset":58512,"ancestor_record_offset":57872,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":58560,"parent_record_offset":58536,"ancestor_record_offset":57872,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":58584,"parent_record_offset":58560,"ancestor_record_offset":57872,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":58512,"parent_record_offset":58448,"ancestor_record_offset":57872,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":59028,"parent_record_offset":58988,"ancestor_record_offset":57872,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":58480,"parent_record_offset":58448,"ancestor_record_offset":57872,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":58448,"parent_record_offset":58420,"ancestor_record_offset":57872,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":59076,"parent_record_offset":59052,"ancestor_record_offset":57872,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":59052,"parent_record_offset":59028,"ancestor_record_offset":57872,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":58420,"parent_record_offset":58392,"ancestor_record_offset":57872,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":58988,"parent_record_offset":58956,"ancestor_record_offset":57872,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":683,"record_offset":58204,"parent_record_offset":58172,"ancestor_record_offset":57872,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":683,"record_offset":58768,"parent_record_offset":58740,"ancestor_record_offset":57872,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":58228,"parent_record_offset":58204,"ancestor_record_offset":57872,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":58076,"parent_record_offset":58048,"ancestor_record_offset":57872,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::effective_mod_root::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 512
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17490
// 首条位置: {"module":2045,"inlinee":95376}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2045,"record_offset":13232,"parent_record_offset":13204,"ancestor_record_offset":13204,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2045,"record_offset":13340,"parent_record_offset":13304,"ancestor_record_offset":13204,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2045,"record_offset":13304,"parent_record_offset":13280,"ancestor_record_offset":13204,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2045,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":13204,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2045,"record_offset":13372,"parent_record_offset":13340,"ancestor_record_offset":13204,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2045,"record_offset":13400,"parent_record_offset":13340,"ancestor_record_offset":13204,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2045,"record_offset":13432,"parent_record_offset":13304,"ancestor_record_offset":13204,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/02045.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::effective_mod_root::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 513
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17491
// 首条位置: {"module":2045,"inlinee":95377}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::trim_mod_cache
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 532
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1D6
// 首条位置: {"module":683,"inlinee":45526}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":683,"record_offset":15600,"parent_record_offset":12616,"ancestor_record_offset":12616,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":683,"record_offset":13668,"parent_record_offset":12616,"ancestor_record_offset":12616,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":13640,"parent_record_offset":12616,"ancestor_record_offset":12616,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":15400,"parent_record_offset":12616,"ancestor_record_offset":12616,"inlinee":15644,"source_line":1559,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":683,"record_offset":13920,"parent_record_offset":12616,"ancestor_record_offset":12616,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":683,"record_offset":15516,"parent_record_offset":12616,"ancestor_record_offset":12616,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":14588,"parent_record_offset":12616,"ancestor_record_offset":12616,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":12788,"parent_record_offset":12616,"ancestor_record_offset":12616,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":15656,"parent_record_offset":12616,"ancestor_record_offset":12616,"inlinee":13843,"source_line":3232,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":14560,"parent_record_offset":12616,"ancestor_record_offset":12616,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::discovery::mod_cache_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":12728,"parent_record_offset":12616,"ancestor_record_offset":12616,"inlinee":45527,"source_line":21,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":13148,"parent_record_offset":13124,"ancestor_record_offset":12616,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":13172,"parent_record_offset":13148,"ancestor_record_offset":12616,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":13196,"parent_record_offset":13172,"ancestor_record_offset":12616,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":13124,"parent_record_offset":13060,"ancestor_record_offset":12616,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":13092,"parent_record_offset":13060,"ancestor_record_offset":12616,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":13060,"parent_record_offset":13032,"ancestor_record_offset":12616,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":13032,"parent_record_offset":13004,"ancestor_record_offset":12616,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":14424,"parent_record_offset":14388,"ancestor_record_offset":12616,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":683,"record_offset":14388,"parent_record_offset":14364,"ancestor_record_offset":12616,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":683,"record_offset":15680,"parent_record_offset":15656,"ancestor_record_offset":12616,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":13692,"parent_record_offset":13668,"ancestor_record_offset":12616,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":13748,"parent_record_offset":13716,"ancestor_record_offset":12616,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":683,"record_offset":13716,"parent_record_offset":13692,"ancestor_record_offset":12616,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":13804,"parent_record_offset":13776,"ancestor_record_offset":12616,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":683,"record_offset":13776,"parent_record_offset":13748,"ancestor_record_offset":12616,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":683,"record_offset":15624,"parent_record_offset":15600,"ancestor_record_offset":12616,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=52
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":12864,"parent_record_offset":12788,"ancestor_record_offset":12616,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":683,"record_offset":14456,"parent_record_offset":14424,"ancestor_record_offset":12616,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":12828,"parent_record_offset":12788,"ancestor_record_offset":12616,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":683,"record_offset":13832,"parent_record_offset":13804,"ancestor_record_offset":12616,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":15540,"parent_record_offset":15516,"ancestor_record_offset":12616,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":15564,"parent_record_offset":15540,"ancestor_record_offset":12616,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":14484,"parent_record_offset":14424,"ancestor_record_offset":12616,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":683,"record_offset":14516,"parent_record_offset":14388,"ancestor_record_offset":12616,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":15452,"parent_record_offset":15428,"ancestor_record_offset":12616,"inlinee":18112,"source_line":2575,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":12752,"parent_record_offset":12728,"ancestor_record_offset":12616,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":683,"record_offset":15476,"parent_record_offset":15452,"ancestor_record_offset":12616,"inlinee":18113,"source_line":188,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::discovery::trim_mod_cache::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/discovery.rs
//     evidence={"module":683,"record_offset":15428,"parent_record_offset":15400,"ancestor_record_offset":12616,"inlinee":45529,"source_line":545,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::discovery::trim_mod_cache::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 545
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1D9
// 首条位置: {"module":683,"inlinee":45529}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":683,"record_offset":15452,"parent_record_offset":15428,"ancestor_record_offset":15428,"inlinee":18112,"source_line":2575,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":683,"record_offset":15476,"parent_record_offset":15452,"ancestor_record_offset":15428,"inlinee":18113,"source_line":188,"archive":"_pdb/C/modules/00683.jsonl.gz"}
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
// 完整结构及原始类型记录: discovery.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: discovery.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: discovery.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<walkdir::dent::DirEntry> > (*)(enum2$<core::result::Result<walkdir::dent::DirEntry,walkdir::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<walkdir::dent::DirEntry,walkdir::error::Error> >"]

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::deleted_mods_root
// 信息 ID: 6fbf34f60173c084f10653c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::shelve_mod; module=683, inline_record=2032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::deleted_mods_root::closure$0
// 信息 ID: ca1054a940c55ab423a420cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::deleted_mods_root", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::shelve_mod; module=683, inline_record=2316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::deleted_mods_root::closure$1
// 信息 ID: af4b8493e3a5a05f3ca7bfbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_mods::discovery::deleted_mods_root", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::shelve_mod; module=683, inline_record=2408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::discover_mods
// 信息 ID: 0b0c8e6eebeb850cca45ff49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::mod_detection::impl$0::build @ 0x5382d5 -> 0xd8fb90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::effective_mod_root
// 信息 ID: b45a1c58b2466084ae840ed1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_mods::discovery::scan_mods_root @ 0xd9167f -> 0xd943e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::scan_mods_root @ 0xd92266 -> 0xd943e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<std::path::PathBuf> >,tg_mods::discovery::partless_mod_folders::closure_env$1> @ 0x26fee0b -> 0xd943e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::effective_mod_root::closure$0
// 信息 ID: 25ea94bf6e23e159c3797225
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5008ae74692ef3f813781806: enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::effective_mod_root", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::effective_mod_root::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::effective_mod_root::closure_env$0>,tg_mods::discovery::effective_mod_root::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::effective_mod_root::closure_env$0 <- alloc::vec::spec_extend::impl$0::spec_extend<std::path::PathBuf,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::effective_mod_root::closure_env$0>,tg_mods::discovery::effective_mod_root::closure_env$1>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::effective_mod_root::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::effective_mod_root::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,ref_mut$<tg_mods::discovery::effective_mod_root::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::effective_mod_root::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,ref_mut$<tg_mods::discovery::effective_mod_root::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; module=2045, inline_record=13204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::effective_mod_root::closure$1
// 信息 ID: 0cc433d46ce2c254b44853c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5008ae74692ef3f813781806: enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_mods::discovery::effective_mod_root", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::effective_mod_root::closure_env$1 <- core::ptr::drop_in_place<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::effective_mod_root::closure_env$0>,tg_mods::discovery::effective_mod_root::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::effective_mod_root::closure_env$1 <- alloc::vec::spec_extend::impl$0::spec_extend<std::path::PathBuf,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::effective_mod_root::closure_env$0>,tg_mods::discovery::effective_mod_root::closure_env$1>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::effective_mod_root::closure_env$1 <- core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::effective_mod_root::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,ref_mut$<tg_mods::discovery::effective_mod_root::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::effective_mod_root::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,ref_mut$<tg_mods::discovery::effective_mod_root::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; module=2045, inline_record=13788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::extracted_zip_root
// 信息 ID: 64962568d82715c9058ecc86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::scan_mods_root; module=683, inline_record=27832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::extracted_zip_root::closure$0
// 信息 ID: 002a94a4c3a547ad9ab88d56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::extracted_zip_root", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::extracted_zip_root::closure_env$0 <- anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,tg_mods::discovery::extracted_zip_root::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,tg_mods::discovery::extracted_zip_root::closure_env$0>; module=2046, inline_record=2184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::extracted_zip_root::closure$1
// 信息 ID: 045ded64ee5230dd6c3a94e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_mods::discovery::extracted_zip_root", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::extracted_zip_root::closure_env$1 <- anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,tg_mods::discovery::extracted_zip_root::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,tg_mods::discovery::extracted_zip_root::closure_env$1>; module=2046, inline_record=780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::has_exact_entry
// 信息 ID: 32090cc61d402ead55d22209
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_mods::discovery::mod_parts @ 0xd94f5f -> 0xd93a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::mod_parts @ 0xd94cd9 -> 0xd93a90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::has_exact_entry::closure$0
// 信息 ID: e578219c55a0d35419876130
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::has_exact_entry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::has_exact_entry; module=683, inline_record=49792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::has_exact_entry::closure$0::closure$0
// 信息 ID: 586c904c18ac3d3aae795694
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::has_exact_entry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::has_exact_entry; module=683, inline_record=51296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::log_partless
// 信息 ID: 170b64f7dd5ef98165ee918f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::mod_at_root; module=683, inline_record=6508; 不是独立调用边。
// [PDB 内联宿主] tg_mods::discovery::scan_mods_root; module=683, inline_record=38676; 不是独立调用边。
// [PDB 内联宿主] tg_mods::discovery::scan_mods_root; module=683, inline_record=41672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::mod_at_root
// 信息 ID: 135c2449b3a06e01208b86cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::steam::workshop_mods::register_workshop_mod @ 0xafd9c0 -> 0xd8f460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::mod_at_root::closure$0
// 信息 ID: e3d6335ab86322490bda3e24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::mod_at_root", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::mod_at_root; module=683, inline_record=7508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::mod_cache_root
// 信息 ID: d34e831e1aea32e2374698a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::discover_mods; module=683, inline_record=12728; 不是独立调用边。
// [PDB 内联宿主] tg_mods::discovery::scan_mods_root; module=683, inline_record=28228; 不是独立调用边。
// [PDB 内联宿主] tg_mods::discovery::scan_mods_root; module=683, inline_record=29176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::mod_parts
// 信息 ID: e85e3226793089071fd2bd3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::mod_editor::mod_editor_tick @ 0xaa1f98 -> 0xd94a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::validate_for_publish @ 0xb00bb8 -> 0xd94a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2dc06 -> 0xd94a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::mod_at_root @ 0xd8f5dd -> 0xd94a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::effective_mod_root @ 0xd94428 -> 0xd94a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::effective_mod_root @ 0xd9454e -> 0xd94a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::discover_clutter_part @ 0x1add316 -> 0xd94a50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::mod_parts::closure$1
// 信息 ID: 01da76a5a5c05eb9a9c08b26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_mods::discovery::mod_parts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::mod_parts; module=683, inline_record=73108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::mod_parts::closure$2
// 信息 ID: a84488735a2f1b44a3e87bda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_mods::discovery::mod_parts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::mod_parts; module=683, inline_record=67492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::mods_root
// 信息 ID: d8813d24fea21d23cf16809f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::mod_editor::ModEditor::create_mod @ 0xaa6b2d -> 0xd96190 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_mods::discovery::shelve_mod; module=683, inline_record=2092; 不是独立调用边。
// [PDB 内联宿主] tg_mods::discovery::discover_mods; module=683, inline_record=10216; 不是独立调用边。
// [PDB 内联宿主] tg_mods::discovery::rediscover_mods; module=683, inline_record=53512; 不是独立调用边。
// [PDB 内联宿主] tg_mods::discovery::partless_mod_folders; module=683, inline_record=61188; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::mods_root::closure$0
// 信息 ID: 54779d769ed4a625691ecc06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::mods_root", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::shelve_mod; module=683, inline_record=2224; 不是独立调用边。
// [PDB 内联宿主] tg_mods::discovery::discover_mods; module=683, inline_record=10340; 不是独立调用边。
// [PDB 内联宿主] tg_mods::discovery::rediscover_mods; module=683, inline_record=53648; 不是独立调用边。
// [PDB 内联宿主] tg_mods::discovery::partless_mod_folders; module=683, inline_record=61312; 不是独立调用边。
// [PDB 内联宿主] tg_mods::discovery::mods_root; module=683, inline_record=77512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::partless_mod_folders
// 信息 ID: 82fa24c7b94e61f6c3dcfd38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::mods::ui @ 0x234f0fa -> 0xd94800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::partless_mod_folders::closure$0
// 信息 ID: ebdc13f6d123040c0ee0249f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5008ae74692ef3f813781806: enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::partless_mod_folders", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::partless_mod_folders::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<std::path::PathBuf,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::partless_mod_folders::closure_env$0>,tg_mods::discovery::partless_mod_folders::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::partless_mod_folders::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::partless_mod_folders::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,ref_mut$<tg_mods::discovery::partless_mod_folders::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::partless_mod_folders::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,ref_mut$<tg_mods::discovery::partless_mod_folders::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; module=2045, inline_record=8532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::partless_mod_folders::closure$1
// 信息 ID: 8bd7661db4cb9c9d65db71c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5008ae74692ef3f813781806: enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_mods::discovery::partless_mod_folders", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::partless_mod_folders::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<std::path::PathBuf,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::partless_mod_folders::closure_env$0>,tg_mods::discovery::partless_mod_folders::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::partless_mod_folders::closure_env$1 <- core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::partless_mod_folders::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,ref_mut$<tg_mods::discovery::partless_mod_folders::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::partless_mod_folders::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<std::path::PathBuf> >,tg_mods::discovery::partless_mod_folders::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<std::path::PathBuf> >,tg_mods::discovery::partless_mod_folders::closure_env$1>; module=3559, inline_record=484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::rediscover_mods
// 信息 ID: 798e76cdfcc552ac2b7f82a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::mod_editor::mod_editor_tick @ 0xaa1e5d -> 0xd93dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::mod_editor::mod_editor_tick @ 0xaa39fd -> 0xd93dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::mods::ui @ 0x234ecf3 -> 0xd93dc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::rediscover_mods::closure$1
// 信息 ID: d1e325c67e4f2093511ae377
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_mods::discovery::rediscover_mods", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::rediscover_mods::closure_env$1 <- alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter::filter_fold::closure_env$0<tg_mods::overlay::Mod,tuple$<>,tg_mods::discovery::rediscover_mods::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<tg_mods::overlay::Mod,alloc::string::String,tuple$<>,tg_mods::discovery::rediscover_mods::closure_env$1,core::iter::adapters::map::map_fold::closure_env$0<alloc::string::String,tuple$<alloc::string::String,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1> > > > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::rediscover_mods::closure_env$1 <- alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter::filter_fold::closure_env$0<tg_mods::overlay::Mod,tuple$<>,tg_mods::discovery::rediscover_mods::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<tg_mods::overlay::Mod,alloc::string::String,tuple$<>,tg_mods::discovery::rediscover_mods::closure_env$1,core::iter::adapters::map::map_fold::closure_env$0<alloc::string::String,tuple$<alloc::string::String,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1> > > > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::rediscover_mods::closure_env$1 <- alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter::filter_fold::closure_env$0<tg_mods::overlay::Mod,tuple$<>,tg_mods::discovery::rediscover_mods::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<tg_mods::overlay::Mod,alloc::string::String,tuple$<>,tg_mods::discovery::rediscover_mods::closure_env$1,core::iter::adapters::map::map_fold::closure_env$0<alloc::string::String,tuple$<alloc::string::String,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1> > > > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::rediscover_mods::closure_env$1 <- alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter::filter_fold::closure_env$0<tg_mods::overlay::Mod,tuple$<>,tg_mods::discovery::rediscover_mods::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<tg_mods::overlay::Mod,alloc::string::String,tuple$<>,tg_mods::discovery::rediscover_mods::closure_env$1,core::iter::adapters::map::map_fold::closure_env$0<alloc::string::String,tuple$<alloc::string::String,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1> > > > > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter::filter_fold::closure_env$0<tg_mods::overlay::Mod,tuple$<>,tg_mods::discovery::rediscover_mods::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<tg_mods::overlay::Mod,alloc::string::String,tuple$<>,tg_mods::discovery::rediscover_mods::closure_env$1,core::iter::adapters::map::map_fold::closure_env$0<alloc::string::String,tuple$<alloc::string::String,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1>,hashbrown::set::impl$10::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,tg_mods::discovery::rediscover_mods::closure_env$0>,tg_mods::discovery::rediscover_mods::closure_env$1> > > > > > > > >; module=2052, inline_record=5012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::sanitized_zip_entry_path
// 信息 ID: 4ae89ac6f9353679b681c9e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::scan_mods_root; module=683, inline_record=31836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::sanitized_zip_entry_path::closure$1
// 信息 ID: ff41a7bb3dc60d32409e43e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_mods::discovery::sanitized_zip_entry_path", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::scan_mods_root; module=683, inline_record=31948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::scan_mods_root
// 信息 ID: bf53b250edf713ffd03086fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_mods::discovery::discover_mods @ 0xd8fcc8 -> 0xd91160 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::rediscover_mods @ 0xd93f1c -> 0xd91160 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::scan_mods_root::closure$0
// 信息 ID: efd27b41e3ba0e689c4a9920
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::scan_mods_root", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::scan_mods_root; module=683, inline_record=25656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::scan_mods_root::closure$1
// 信息 ID: 0b4a1d7f2e9b10c7a47e3b79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_mods::discovery::scan_mods_root", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::scan_mods_root; module=683, inline_record=25820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::scan_mods_root::closure$2
// 信息 ID: aa11a0991e2a6634bc840597
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_mods::discovery::scan_mods_root", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::scan_mods_root; module=683, inline_record=26748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::scan_mods_root::closure$3
// 信息 ID: 00afd5aca8a7fbce79261b44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tg_mods::discovery::scan_mods_root", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::scan_mods_root::closure_env$3 <- alloc::vec::Vec::retain<tg_mods::overlay::Mod,alloc::alloc::Global,tg_mods::discovery::scan_mods_root::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<tg_mods::overlay::Mod,alloc::alloc::Global,tg_mods::discovery::scan_mods_root::closure_env$3>; module=2029, inline_record=4076; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<tg_mods::overlay::Mod,alloc::alloc::Global,tg_mods::discovery::scan_mods_root::closure_env$3>; module=2029, inline_record=4508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::shelve_mod
// 信息 ID: 9ddf33f98974eebd08feb8ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::shelve_mod::closure$0
// 信息 ID: 5b56987991d671eb2e5caef6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::shelve_mod", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::shelve_mod; module=683, inline_record=1920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::shelve_mod::closure$1
// 信息 ID: b17bf846f734b7cc65827315
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_mods::discovery::shelve_mod", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::shelve_mod::closure_env$1 <- core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::ops::range::RangeFrom<i32>,tg_mods::discovery::shelve_mod::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,tg_mods::discovery::shelve_mod::closure_env$2>,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::ops::range::RangeFrom<i32>,tg_mods::discovery::shelve_mod::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,tg_mods::discovery::shelve_mod::closure_env$2>,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; module=2045, inline_record=10568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::shelve_mod::closure$2
// 信息 ID: 514f4b037fed726d8e5b63a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_mods::discovery::shelve_mod", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::shelve_mod::closure_env$2 <- core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::ops::range::RangeFrom<i32>,tg_mods::discovery::shelve_mod::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,tg_mods::discovery::shelve_mod::closure_env$2>,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::ops::range::RangeFrom<i32>,tg_mods::discovery::shelve_mod::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,tg_mods::discovery::shelve_mod::closure_env$2>,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; module=2045, inline_record=11364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::shelve_mod::closure$3
// 信息 ID: 191737232520db26d2eb7f01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tg_mods::discovery::shelve_mod", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::shelve_mod::closure_env$3 <- anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,tg_mods::discovery::shelve_mod::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,tg_mods::discovery::shelve_mod::closure_env$3>; module=2046, inline_record=1476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::thumbnail_path
// 信息 ID: 0ff2a4af9b8f98a529e5bb75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::mod_editor::mod_editor_tick @ 0xaa1fe3 -> 0xd939d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::author_preview_path @ 0xb0032f -> 0xd939d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::validate_for_publish @ 0xb00c7f -> 0xd939d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0 @ 0x2a447bc -> 0xd939d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_mods::discovery::mod_at_root; module=683, inline_record=8228; 不是独立调用边。
// [PDB 内联宿主] tg_mods::discovery::scan_mods_root; module=683, inline_record=39260; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::thumbnail_path::closure$0
// 信息 ID: 5491b2141569623b454b9a60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::thumbnail_path", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::thumbnail_path::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::array::iter::IntoIter<ref$<str$>,3>,tg_mods::discovery::thumbnail_path::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,tg_mods::discovery::thumbnail_path::closure_env$1>,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::array::iter::IntoIter<ref$<str$>,3>,tg_mods::discovery::thumbnail_path::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,tg_mods::discovery::thumbnail_path::closure_env$1>,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; module=2045, inline_record=7368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::thumbnail_path::closure$1
// 信息 ID: 23013fc070f4d832e8102db2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_mods::discovery::thumbnail_path", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::thumbnail_path::closure_env$1 <- core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::array::iter::IntoIter<ref$<str$>,3>,tg_mods::discovery::thumbnail_path::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,tg_mods::discovery::thumbnail_path::closure_env$1>,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<std::path::PathBuf,core::array::iter::IntoIter<ref$<str$>,3>,tg_mods::discovery::thumbnail_path::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::path::PathBuf,tg_mods::discovery::thumbnail_path::closure_env$1>,enum2$<core::ops::control_flow::ControlFlow<std::path::PathBuf,tuple$<> > > >; module=2045, inline_record=6600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::touch_last_used
// 信息 ID: b94374f6a386320c2dcd0774
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_mods::discovery::scan_mods_root @ 0xd91a16 -> 0xd94220 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::scan_mods_root @ 0xd92887 -> 0xd94220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::trim_mod_cache
// 信息 ID: 8e7f9b82179d6ae4daca18df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::discover_mods; module=683, inline_record=12616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::trim_mod_cache::closure$0
// 信息 ID: ce465ec6ae769d4a4d03decd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::trim_mod_cache", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::discover_mods; module=683, inline_record=15428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::trim_orphaned_zip_sidecars
// 信息 ID: 527451d1908f573e2e10e5dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::discovery::discover_mods; module=683, inline_record=17616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::trim_orphaned_zip_sidecars::closure$0
// 信息 ID: e99b49b6cd0000ae4c1a19a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5008ae74692ef3f813781806: enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::trim_orphaned_zip_sidecars", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$0>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$0>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$1> >; module=2029, inline_record=31360; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$0>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$1> >; module=2029, inline_record=33228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::trim_orphaned_zip_sidecars::closure$0::closure$0
// 信息 ID: f6c0578a62d94da719a92494
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5008ae74692ef3f813781806: enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::discovery::trim_orphaned_zip_sidecars", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$0>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$1> >; module=2029, inline_record=31504; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$0>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$1> >; module=2029, inline_record=33372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::trim_orphaned_zip_sidecars::closure$1
// 信息 ID: d6ea1b06cd0f6c1ec6f80a2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5008ae74692ef3f813781806: enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_mods::discovery::trim_orphaned_zip_sidecars", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$0>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<std::fs::DirEntry>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$1>; module=3559, inline_record=2312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::trim_orphaned_zip_sidecars::closure$2
// 信息 ID: c93d5892d72ddb87d36f5ffa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_mods::discovery::trim_orphaned_zip_sidecars", "captures": "unknown"}
// [闭包环境类型引用] tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,tuple$<> >,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<ref$<str$>,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,tuple$<> >,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<ref$<str$>,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,tuple$<> >,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<ref$<str$>,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,tuple$<> >,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<ref$<str$>,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,tuple$<> >,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<ref$<str$>,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> > > > > >; module=2045, inline_record=1724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::trim_orphaned_zip_sidecars::closure$2::closure$0
// 信息 ID: 3fa3af86d0bcbe41a20b9786
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_mods::discovery::trim_orphaned_zip_sidecars", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,tuple$<> >,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<ref$<str$>,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> > > > > >; module=2045, inline_record=1808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::discovery::trim_orphaned_zip_sidecars::closure$2::closure$1
// 信息 ID: 0676c89f48cbd1ebea144fb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_mods::discovery::trim_orphaned_zip_sidecars", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,tuple$<> >,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<ref$<str$>,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2>,hashbrown::set::impl$10::extend::closure_env$0<ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<alloc::string::String>,tg_mods::discovery::trim_orphaned_zip_sidecars::closure_env$2> > > > > >; module=2045, inline_record=1860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
