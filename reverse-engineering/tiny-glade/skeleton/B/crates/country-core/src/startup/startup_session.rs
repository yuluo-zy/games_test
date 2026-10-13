// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/startup_session.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e325e97c96c3744be0861aab2fcf2a9d006ea64d605a23af78f22c0df4f17712
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_session.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1629, 1644, 1696
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.15.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_game_session
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1644,"record_offset":25184,"rva":23521232,"va":5392230352}
// 编译实例 RVAs: 0x166e7d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..collections..vec_deque..VecDeque$LT$T$C$A$GT$$u20$as$u20$alloc..collections..vec_deque..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$::spec_extend::habe0c8679b8a6e5d (.llvm.17652999717976659393)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11626142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e862994eff","size":5,"soft_fail":false,"target_va":5380612480,"va":5392238617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9384849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e86fcc70ff","size":5,"soft_fail":false,"target_va":5382847888,"va":5392232732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-9586523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8a5b86dff","size":5,"soft_fail":false,"target_va":5382644528,"va":5392231046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9527218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e84ea06eff","size":5,"soft_fail":false,"target_va":5382708864,"va":5392236077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9468782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e892846fff","size":5,"soft_fail":false,"target_va":5382768048,"va":5392236825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014b4d0
//     owner=unresolved_namespace; count=6
//     target_aliases=14 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22177498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e82699adfe","size":5,"soft_fail":false,"target_va":5370066128,"va":5392243621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5d80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20490942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e84255c7fe","size":5,"soft_fail":false,"target_va":5371747712,"va":5392238649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d8fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=23 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17406438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e81a66f6fe","size":5,"soft_fail":false,"target_va":5374840768,"va":5392247201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dd0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17391389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8e3a0f6fe","size":5,"soft_fail":false,"target_va":5374857392,"va":5392248776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e69f0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17352077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8733af7fe","size":5,"soft_fail":false,"target_va":5374896624,"va":5392248696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17345449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e85754f7fe","size":5,"soft_fail":false,"target_va":5374900736,"va":5392246180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17150696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8184dfafe","size":5,"soft_fail":false,"target_va":5375081488,"va":5392232179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=58
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17150181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e81b4ffafe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392231680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=9
//     target_aliases=69 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16714827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8b5f300ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5392230854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140876a30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14661576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8384820ff","size":5,"soft_fail":false,"target_va":5377583664,"va":5392245235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140878540
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14648086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8ea7c20ff","size":5,"soft_fail":false,"target_va":5377590592,"va":5392238673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087cb00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14633708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e814b520ff","size":5,"soft_fail":false,"target_va":5377608448,"va":5392242151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088fa00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14557373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e843df21ff","size":5,"soft_fail":false,"target_va":5377686016,"va":5392243384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140890130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14551941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e87bf421ff","size":5,"soft_fail":false,"target_va":5377687856,"va":5392239792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898020
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14521785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8476a22ff","size":5,"soft_fail":false,"target_va":5377720352,"va":5392242132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b4c390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11682519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e829bd4dff","size":5,"soft_fail":false,"target_va":5380555664,"va":5392238178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9531519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8818f6eff","size":5,"soft_fail":false,"target_va":5382704752,"va":5392236266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=12
//     target_aliases=5 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9403854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8328270ff","size":5,"soft_fail":false,"target_va":5382827728,"va":5392231577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413320d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3401072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8901accff","size":5,"soft_fail":false,"target_va":5388837072,"va":5392238139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146ca50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2119764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8aca7dfff","size":5,"soft_fail":false,"target_va":5390125648,"va":5392245407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a5b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-827990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8aa5df3ff","size":5,"soft_fail":false,"target_va":5391407968,"va":5392235953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141740c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t850718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e81efb0c00","size":5,"soft_fail":false,"target_va":5393091584,"va":5392240861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19616648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e888532b01","size":5,"soft_fail":false,"target_va":5411847168,"va":5392230515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19615211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8eb4d2b01","size":5,"soft_fail":false,"target_va":5411847184,"va":5392231968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19298283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8eb772601","size":5,"soft_fail":false,"target_va":5411540992,"va":5392242704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11616609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e89fbe4eff","size":5,"soft_fail":false,"target_va":5380615296,"va":5392231900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-17875539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8ad3deffe","size":5,"soft_fail":false,"target_va":5374355568,"va":5392231102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19104801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e821842301","size":5,"soft_fail":false,"target_va":5411347040,"va":5392242234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19298113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e841772601","size":5,"soft_fail":false,"target_va":5411540963,"va":5392242845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-9552904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8f83b6eff","size":5,"soft_fail":false,"target_va":5382683584,"va":5392236483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9585511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e899bc6dff","size":5,"soft_fail":false,"target_va":5382655024,"va":5392240530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-3739051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e855f2c6ff","size":5,"soft_fail":false,"target_va":5388503056,"va":5392242102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19299049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8e97a2601","size":5,"soft_fail":false,"target_va":5411541808,"va":5392242754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19299477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8957c2601","size":5,"soft_fail":false,"target_va":5411541776,"va":5392242294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19299789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8cd7d2601","size":5,"soft_fail":false,"target_va":5411542064,"va":5392242270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19300916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e834822601","size":5,"soft_fail":false,"target_va":5411543584,"va":5392242663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19299526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8c67c2601","size":5,"soft_fail":false,"target_va":5411541904,"va":5392242373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$country_core..startup..startup_session..SessionLoadAttempt$GT$::hc8f8df096e02499e (.llvm.17652999717976659393)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11629620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8cc8b4eff","size":5,"soft_fail":false,"target_va":5380613584,"va":5392243199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> > > >
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-5700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8bce9ffff","size":5,"soft_fail":false,"target_va":5392229808,"va":5392235503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17147863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e82958fafe","size":5,"soft_fail":false,"target_va":5375099104,"va":5392246962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::save::load_game::PostGameLoadCommand> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-14747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e865c6ffff","size":5,"soft_fail":false,"target_va":5392230288,"va":5392245030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19300447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e85f802601","size":5,"soft_fail":false,"target_va":5411542880,"va":5392242428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::glade_settings::GladeSettings::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14374939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8e5a724ff","size":5,"soft_fail":false,"target_va":5377866688,"va":5392241622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11701394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e86e734dff","size":5,"soft_fail":false,"target_va":5380530176,"va":5392231565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::try_load_from_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11708726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8ca564dff","size":5,"soft_fail":false,"target_va":5380531328,"va":5392240049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::write_to_disk
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11703310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8f26b4dff","size":5,"soft_fail":false,"target_va":5380531136,"va":5392234441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_completion_checker::is_session_ok_to_load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12381959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8f91043ff","size":5,"soft_fail":false,"target_va":5379848832,"va":5392230786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9524763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8e5a96eff","size":5,"soft_fail":false,"target_va":5382709584,"va":5392234342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9527157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e88ba06eff","size":5,"soft_fail":false,"target_va":5382709008,"va":5392236160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-9487536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8503b6fff","size":5,"soft_fail":false,"target_va":5382744080,"va":5392231611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9494733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8331f6fff","size":5,"soft_fail":false,"target_va":5382741504,"va":5392236232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9490579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e86d2f6fff","size":5,"soft_fail":false,"target_va":5382745616,"va":5392236190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9496468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e86c186fff","size":5,"soft_fail":false,"target_va":5382741200,"va":5392237663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9464146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8ae966fff","size":5,"soft_fail":false,"target_va":5382770560,"va":5392234701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9458369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e83fad6fff","size":5,"soft_fail":false,"target_va":5382776592,"va":5392234956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10875943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392230352,"raw_hex":"e8d90b5aff","size":5,"soft_fail":false,"target_va":5381358304,"va":5392234242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5392230352,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5392235039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":47268,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1644,"record_offset":25968,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4596,"source_line":558,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1644,"record_offset":36168,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4905,"source_line":1698,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1644,"record_offset":28200,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4733,"source_line":1827,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1644,"record_offset":47420,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1644,"record_offset":42420,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":6034,"source_line":238,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1644,"record_offset":42392,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1644,"record_offset":37996,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1644,"record_offset":39036,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1644,"record_offset":42736,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1644,"record_offset":46300,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1644,"record_offset":25792,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":1644,"record_offset":26376,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":1644,"record_offset":26988,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":9992,"source_line":95,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::remove_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1644,"record_offset":25528,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4958,"source_line":872,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1644,"record_offset":38832,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1644,"record_offset":26168,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1644,"record_offset":53152,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1644,"record_offset":42796,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":9207,"source_line":347,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1644,"record_offset":25852,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1644,"record_offset":42708,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1644,"record_offset":26024,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1644,"record_offset":46208,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1644,"record_offset":51840,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1644,"record_offset":46000,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1644,"record_offset":32588,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1644,"record_offset":38768,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":31620,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1644,"record_offset":33888,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1644,"record_offset":37924,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1644,"record_offset":50068,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1644,"record_offset":25996,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":4740,"source_line":153,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] country_core::meta_commander::MetaCommandFence::unblock
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/meta_commander.rs
//     evidence={"module":1644,"record_offset":46632,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":28798,"source_line":117,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] country_core::meta_commander::MetaCommander::extend_script
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/meta_commander.rs
//     evidence={"module":1644,"record_offset":46520,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":31903,"source_line":147,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1644,"record_offset":52444,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":33516,"source_line":264,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::SessionId::snapshots_folder_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1644,"record_offset":30068,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":33507,"source_line":151,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1644,"record_offset":52244,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":26643,"source_line":198,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$28::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1644,"record_offset":28568,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":13520,"source_line":181,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1644,"record_offset":30896,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":74809,"source_line":294,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_session::MaybeExists::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1644,"record_offset":34428,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":74811,"source_line":39,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_session::SessionLoadAttempt::from_session_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1644,"record_offset":27356,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":74807,"source_line":79,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_session::load_session_meta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1644,"record_offset":48460,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":74814,"source_line":438,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::load_completion_checker::SessionLoadReadiness::is_ok
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/load_completion_checker.rs
//     evidence={"module":1644,"record_offset":26056,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":28248,"source_line":73,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1644,"record_offset":26200,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1644,"record_offset":26084,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":38804,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1644,"record_offset":32996,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1644,"record_offset":34360,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":44888,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":13003,"source_line":3025,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":37896,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":38488,"parent_record_offset":25184,"ancestor_record_offset":25184,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":29152,"parent_record_offset":29128,"ancestor_record_offset":25184,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":29176,"parent_record_offset":29152,"ancestor_record_offset":25184,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":27820,"parent_record_offset":27796,"ancestor_record_offset":25184,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":29128,"parent_record_offset":28944,"ancestor_record_offset":25184,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":27796,"parent_record_offset":27740,"ancestor_record_offset":25184,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1644,"record_offset":35064,"parent_record_offset":35036,"ancestor_record_offset":25184,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1644,"record_offset":36404,"parent_record_offset":36168,"ancestor_record_offset":25184,"inlinee":4907,"source_line":181,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1644,"record_offset":28268,"parent_record_offset":28244,"ancestor_record_offset":25184,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1644,"record_offset":36212,"parent_record_offset":36168,"ancestor_record_offset":25184,"inlinee":4906,"source_line":1342,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1644,"record_offset":28244,"parent_record_offset":28200,"ancestor_record_offset":25184,"inlinee":4734,"source_line":204,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1644,"record_offset":36432,"parent_record_offset":36404,"ancestor_record_offset":25184,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1644,"record_offset":28356,"parent_record_offset":28200,"ancestor_record_offset":25184,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1644,"record_offset":28388,"parent_record_offset":28356,"ancestor_record_offset":25184,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1644,"record_offset":46568,"parent_record_offset":46544,"ancestor_record_offset":25184,"inlinee":32146,"source_line":2952,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1644,"record_offset":46544,"parent_record_offset":46520,"ancestor_record_offset":25184,"inlinee":14993,"source_line":2980,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::into_iter::IntoIter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":1644,"record_offset":46592,"parent_record_offset":46568,"ancestor_record_offset":25184,"inlinee":32148,"source_line":26,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":28292,"parent_record_offset":28268,"ancestor_record_offset":25184,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":26564,"parent_record_offset":26540,"ancestor_record_offset":25184,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":25644,"parent_record_offset":25616,"ancestor_record_offset":25184,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":28876,"parent_record_offset":28844,"ancestor_record_offset":25184,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":25728,"parent_record_offset":25704,"ancestor_record_offset":25184,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":27768,"parent_record_offset":27740,"ancestor_record_offset":25184,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":27740,"parent_record_offset":27716,"ancestor_record_offset":25184,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":25704,"parent_record_offset":25672,"ancestor_record_offset":25184,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":26612,"parent_record_offset":26588,"ancestor_record_offset":25184,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":26588,"parent_record_offset":26564,"ancestor_record_offset":25184,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":25672,"parent_record_offset":25644,"ancestor_record_offset":25184,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":28944,"parent_record_offset":28908,"ancestor_record_offset":25184,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":28908,"parent_record_offset":28876,"ancestor_record_offset":25184,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":27716,"parent_record_offset":27692,"ancestor_record_offset":25184,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":28988,"parent_record_offset":28944,"ancestor_record_offset":25184,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1644,"record_offset":28784,"parent_record_offset":28744,"ancestor_record_offset":25184,"inlinee":11743,"source_line":124,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1644,"record_offset":31128,"parent_record_offset":31100,"ancestor_record_offset":25184,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1644,"record_offset":47444,"parent_record_offset":47420,"ancestor_record_offset":25184,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1644,"record_offset":28744,"parent_record_offset":28668,"ancestor_record_offset":25184,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1644,"record_offset":31072,"parent_record_offset":31044,"ancestor_record_offset":25184,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1644,"record_offset":42448,"parent_record_offset":42420,"ancestor_record_offset":25184,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1644,"record_offset":39656,"parent_record_offset":39620,"ancestor_record_offset":25184,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1644,"record_offset":39620,"parent_record_offset":39596,"ancestor_record_offset":25184,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1644,"record_offset":26540,"parent_record_offset":26500,"ancestor_record_offset":25184,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1644,"record_offset":28716,"parent_record_offset":28668,"ancestor_record_offset":25184,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1644,"record_offset":47468,"parent_record_offset":47444,"ancestor_record_offset":25184,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1644,"record_offset":26500,"parent_record_offset":26464,"ancestor_record_offset":25184,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1644,"record_offset":25616,"parent_record_offset":25584,"ancestor_record_offset":25184,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1644,"record_offset":28844,"parent_record_offset":28784,"ancestor_record_offset":25184,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1644,"record_offset":28628,"parent_record_offset":28568,"ancestor_record_offset":25184,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1644,"record_offset":28668,"parent_record_offset":28628,"ancestor_record_offset":25184,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1644,"record_offset":43420,"parent_record_offset":43396,"ancestor_record_offset":25184,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1644,"record_offset":43844,"parent_record_offset":43820,"ancestor_record_offset":25184,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1644,"record_offset":42828,"parent_record_offset":42796,"ancestor_record_offset":25184,"inlinee":11852,"source_line":239,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1644,"record_offset":43240,"parent_record_offset":42796,"ancestor_record_offset":25184,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1644,"record_offset":25820,"parent_record_offset":25792,"ancestor_record_offset":25184,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1644,"record_offset":26400,"parent_record_offset":26376,"ancestor_record_offset":25184,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":1644,"record_offset":27096,"parent_record_offset":27072,"ancestor_record_offset":25184,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":1644,"record_offset":27072,"parent_record_offset":26988,"ancestor_record_offset":25184,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":1644,"record_offset":26464,"parent_record_offset":26400,"ancestor_record_offset":25184,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1644,"record_offset":25556,"parent_record_offset":25528,"ancestor_record_offset":25184,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1644,"record_offset":25584,"parent_record_offset":25556,"ancestor_record_offset":25184,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1644,"record_offset":35096,"parent_record_offset":35064,"ancestor_record_offset":25184,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1644,"record_offset":29012,"parent_record_offset":28988,"ancestor_record_offset":25184,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1644,"record_offset":29036,"parent_record_offset":29012,"ancestor_record_offset":25184,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1644,"record_offset":50232,"parent_record_offset":50108,"ancestor_record_offset":25184,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1644,"record_offset":38888,"parent_record_offset":38860,"ancestor_record_offset":25184,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1644,"record_offset":31552,"parent_record_offset":31528,"ancestor_record_offset":25184,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1644,"record_offset":42512,"parent_record_offset":42476,"ancestor_record_offset":25184,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1644,"record_offset":29084,"parent_record_offset":29060,"ancestor_record_offset":25184,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1644,"record_offset":29272,"parent_record_offset":29228,"ancestor_record_offset":25184,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":1644,"record_offset":29228,"parent_record_offset":28784,"ancestor_record_offset":25184,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1644,"record_offset":29372,"parent_record_offset":28784,"ancestor_record_offset":25184,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1644,"record_offset":25876,"parent_record_offset":25852,"ancestor_record_offset":25184,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1644,"record_offset":35120,"parent_record_offset":35096,"ancestor_record_offset":25184,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1644,"record_offset":29060,"parent_record_offset":29036,"ancestor_record_offset":25184,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1644,"record_offset":43972,"parent_record_offset":43944,"ancestor_record_offset":25184,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1644,"record_offset":36296,"parent_record_offset":36268,"ancestor_record_offset":25184,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1644,"record_offset":51864,"parent_record_offset":51840,"ancestor_record_offset":25184,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1644,"record_offset":38860,"parent_record_offset":38832,"ancestor_record_offset":25184,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1644,"record_offset":29420,"parent_record_offset":28568,"ancestor_record_offset":25184,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1644,"record_offset":26256,"parent_record_offset":26224,"ancestor_record_offset":25184,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1644,"record_offset":26284,"parent_record_offset":26224,"ancestor_record_offset":25184,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1644,"record_offset":31528,"parent_record_offset":31128,"ancestor_record_offset":25184,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=262
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":27572,"parent_record_offset":27356,"ancestor_record_offset":25184,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1644,"record_offset":28524,"parent_record_offset":28356,"ancestor_record_offset":25184,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1644,"record_offset":34864,"parent_record_offset":34836,"ancestor_record_offset":25184,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1644,"record_offset":34912,"parent_record_offset":34888,"ancestor_record_offset":25184,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1644,"record_offset":34888,"parent_record_offset":34864,"ancestor_record_offset":25184,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1644,"record_offset":43268,"parent_record_offset":43240,"ancestor_record_offset":25184,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1644,"record_offset":39688,"parent_record_offset":39656,"ancestor_record_offset":25184,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1644,"record_offset":43296,"parent_record_offset":43240,"ancestor_record_offset":25184,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1644,"record_offset":29332,"parent_record_offset":29300,"ancestor_record_offset":25184,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":25904,"parent_record_offset":25876,"ancestor_record_offset":25184,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":47376,"parent_record_offset":47348,"ancestor_record_offset":25184,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":25932,"parent_record_offset":25876,"ancestor_record_offset":25184,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1644,"record_offset":34388,"parent_record_offset":34360,"ancestor_record_offset":25184,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1644,"record_offset":38960,"parent_record_offset":38936,"ancestor_record_offset":25184,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1644,"record_offset":38984,"parent_record_offset":38960,"ancestor_record_offset":25184,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1644,"record_offset":29300,"parent_record_offset":29272,"ancestor_record_offset":25184,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":1644,"record_offset":42568,"parent_record_offset":42476,"ancestor_record_offset":25184,"inlinee":7664,"source_line":580,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1644,"record_offset":42476,"parent_record_offset":42448,"ancestor_record_offset":25184,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1644,"record_offset":38912,"parent_record_offset":38888,"ancestor_record_offset":25184,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1644,"record_offset":46656,"parent_record_offset":46632,"ancestor_record_offset":25184,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1644,"record_offset":46800,"parent_record_offset":46764,"ancestor_record_offset":25184,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1644,"record_offset":26108,"parent_record_offset":26084,"ancestor_record_offset":25184,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1644,"record_offset":26132,"parent_record_offset":26108,"ancestor_record_offset":25184,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1644,"record_offset":46680,"parent_record_offset":46656,"ancestor_record_offset":25184,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1644,"record_offset":39716,"parent_record_offset":39656,"ancestor_record_offset":25184,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1644,"record_offset":39748,"parent_record_offset":39620,"ancestor_record_offset":25184,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":1644,"record_offset":29488,"parent_record_offset":29456,"ancestor_record_offset":25184,"inlinee":13524,"source_line":24,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":1644,"record_offset":29456,"parent_record_offset":29420,"ancestor_record_offset":25184,"inlinee":13522,"source_line":27,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":1644,"record_offset":29532,"parent_record_offset":29456,"ancestor_record_offset":25184,"inlinee":13526,"source_line":11,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1644,"record_offset":52268,"parent_record_offset":52244,"ancestor_record_offset":25184,"inlinee":26645,"source_line":53,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$15::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1644,"record_offset":51920,"parent_record_offset":51896,"ancestor_record_offset":25184,"inlinee":10698,"source_line":36,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1644,"record_offset":50440,"parent_record_offset":50108,"ancestor_record_offset":25184,"inlinee":33515,"source_line":242,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_session::load_game_session::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1644,"record_offset":50108,"parent_record_offset":50068,"ancestor_record_offset":25184,"inlinee":74815,"source_line":260,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_session::load_game_session::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1644,"record_offset":46032,"parent_record_offset":46000,"ancestor_record_offset":25184,"inlinee":74812,"source_line":370,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1644,"record_offset":26312,"parent_record_offset":26224,"ancestor_record_offset":25184,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1644,"record_offset":50264,"parent_record_offset":50108,"ancestor_record_offset":25184,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1644,"record_offset":26224,"parent_record_offset":26200,"ancestor_record_offset":25184,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1644,"record_offset":26336,"parent_record_offset":26312,"ancestor_record_offset":25184,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1644,"record_offset":50148,"parent_record_offset":50108,"ancestor_record_offset":25184,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":30960,"parent_record_offset":30932,"ancestor_record_offset":25184,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":27448,"parent_record_offset":27424,"ancestor_record_offset":25184,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":27496,"parent_record_offset":27472,"ancestor_record_offset":25184,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":27472,"parent_record_offset":27448,"ancestor_record_offset":25184,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1644,"record_offset":33228,"parent_record_offset":32996,"ancestor_record_offset":25184,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1644,"record_offset":33032,"parent_record_offset":32996,"ancestor_record_offset":25184,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1644,"record_offset":34660,"parent_record_offset":34624,"ancestor_record_offset":25184,"inlinee":12458,"source_line":2381,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1644,"record_offset":34836,"parent_record_offset":34808,"ancestor_record_offset":25184,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1644,"record_offset":34808,"parent_record_offset":34780,"ancestor_record_offset":25184,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":34624,"parent_record_offset":34428,"ancestor_record_offset":25184,"inlinee":13003,"source_line":3025,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":27540,"parent_record_offset":27356,"ancestor_record_offset":25184,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":27424,"parent_record_offset":27400,"ancestor_record_offset":25184,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":30932,"parent_record_offset":30896,"ancestor_record_offset":25184,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":27400,"parent_record_offset":27356,"ancestor_record_offset":25184,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":33060,"parent_record_offset":32996,"ancestor_record_offset":25184,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1644,"record_offset":30988,"parent_record_offset":30960,"ancestor_record_offset":25184,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":1644,"record_offset":31016,"parent_record_offset":30988,"ancestor_record_offset":25184,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::MaybeExists::get
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1243B
// 首条位置: {"module":1644,"inlinee":74811}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":35560,"parent_record_offset":34428,"ancestor_record_offset":34428,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":34624,"parent_record_offset":34428,"ancestor_record_offset":34428,"inlinee":13003,"source_line":3025,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":34484,"parent_record_offset":34428,"ancestor_record_offset":34428,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":35176,"parent_record_offset":35152,"ancestor_record_offset":34428,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":35152,"parent_record_offset":35064,"ancestor_record_offset":34428,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1644,"record_offset":35064,"parent_record_offset":35036,"ancestor_record_offset":34428,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":35756,"parent_record_offset":35728,"ancestor_record_offset":34428,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":35728,"parent_record_offset":35704,"ancestor_record_offset":34428,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":35704,"parent_record_offset":35680,"ancestor_record_offset":34428,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1644,"record_offset":35096,"parent_record_offset":35064,"ancestor_record_offset":34428,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1644,"record_offset":35120,"parent_record_offset":35096,"ancestor_record_offset":34428,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":34724,"parent_record_offset":34624,"ancestor_record_offset":34428,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1644,"record_offset":34864,"parent_record_offset":34836,"ancestor_record_offset":34428,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1644,"record_offset":34912,"parent_record_offset":34888,"ancestor_record_offset":34428,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1644,"record_offset":34888,"parent_record_offset":34864,"ancestor_record_offset":34428,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1644,"record_offset":34688,"parent_record_offset":34660,"ancestor_record_offset":34428,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":34532,"parent_record_offset":34508,"ancestor_record_offset":34428,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":34580,"parent_record_offset":34556,"ancestor_record_offset":34428,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":34556,"parent_record_offset":34532,"ancestor_record_offset":34428,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1644,"record_offset":34660,"parent_record_offset":34624,"ancestor_record_offset":34428,"inlinee":12458,"source_line":2381,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1644,"record_offset":34836,"parent_record_offset":34808,"ancestor_record_offset":34428,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1644,"record_offset":34808,"parent_record_offset":34780,"ancestor_record_offset":34428,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":34508,"parent_record_offset":34484,"ancestor_record_offset":34428,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::SessionLoadAttempt::from_session_id
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12437
// 首条位置: {"module":1644,"inlinee":74807}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":27572,"parent_record_offset":27356,"ancestor_record_offset":27356,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":27540,"parent_record_offset":27356,"ancestor_record_offset":27356,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":27400,"parent_record_offset":27356,"ancestor_record_offset":27356,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":27820,"parent_record_offset":27796,"ancestor_record_offset":27356,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":27796,"parent_record_offset":27740,"ancestor_record_offset":27356,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":27768,"parent_record_offset":27740,"ancestor_record_offset":27356,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":27740,"parent_record_offset":27716,"ancestor_record_offset":27356,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":27716,"parent_record_offset":27692,"ancestor_record_offset":27356,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":27596,"parent_record_offset":27572,"ancestor_record_offset":27356,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":27448,"parent_record_offset":27424,"ancestor_record_offset":27356,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":27496,"parent_record_offset":27472,"ancestor_record_offset":27356,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":27472,"parent_record_offset":27448,"ancestor_record_offset":27356,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":27424,"parent_record_offset":27400,"ancestor_record_offset":27356,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_game_session::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 260
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1243F
// 首条位置: {"module":1644,"inlinee":74815}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1644,"record_offset":50232,"parent_record_offset":50108,"ancestor_record_offset":50108,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":50468,"parent_record_offset":50108,"ancestor_record_offset":50108,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1644,"record_offset":50440,"parent_record_offset":50108,"ancestor_record_offset":50108,"inlinee":33515,"source_line":242,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1644,"record_offset":50264,"parent_record_offset":50108,"ancestor_record_offset":50108,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1644,"record_offset":50148,"parent_record_offset":50108,"ancestor_record_offset":50108,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1644,"record_offset":50320,"parent_record_offset":50288,"ancestor_record_offset":50108,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1644,"record_offset":50348,"parent_record_offset":50288,"ancestor_record_offset":50108,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1644,"record_offset":50172,"parent_record_offset":50148,"ancestor_record_offset":50108,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1644,"record_offset":50196,"parent_record_offset":50172,"ancestor_record_offset":50108,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1644,"record_offset":50376,"parent_record_offset":50288,"ancestor_record_offset":50108,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1644,"record_offset":50288,"parent_record_offset":50264,"ancestor_record_offset":50108,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1644,"record_offset":50400,"parent_record_offset":50376,"ancestor_record_offset":50108,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_game_session::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x121F7
// 首条位置: {"module":1629,"inlinee":74231}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1629,"record_offset":113884,"parent_record_offset":113864,"ancestor_record_offset":113864,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01629.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1629,"record_offset":113932,"parent_record_offset":113908,"ancestor_record_offset":113864,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01629.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1629,"record_offset":113908,"parent_record_offset":113884,"ancestor_record_offset":113864,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01629.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_game_session::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 370
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1243C
// 首条位置: {"module":1644,"inlinee":74812}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":46056,"parent_record_offset":46032,"ancestor_record_offset":46032,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_session_meta
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 438
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1243E
// 首条位置: {"module":1644,"inlinee":74814}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":48816,"parent_record_offset":48460,"ancestor_record_offset":48460,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":48644,"parent_record_offset":48460,"ancestor_record_offset":48460,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":48504,"parent_record_offset":48460,"ancestor_record_offset":48460,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":49064,"parent_record_offset":49040,"ancestor_record_offset":48460,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":49040,"parent_record_offset":48984,"ancestor_record_offset":48460,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":49012,"parent_record_offset":48984,"ancestor_record_offset":48460,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":48984,"parent_record_offset":48960,"ancestor_record_offset":48460,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1644,"record_offset":48960,"parent_record_offset":48936,"ancestor_record_offset":48460,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":48840,"parent_record_offset":48816,"ancestor_record_offset":48460,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":48552,"parent_record_offset":48528,"ancestor_record_offset":48460,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":48600,"parent_record_offset":48576,"ancestor_record_offset":48460,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1644,"record_offset":48576,"parent_record_offset":48552,"ancestor_record_offset":48460,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":48528,"parent_record_offset":48504,"ancestor_record_offset":48460,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::copy_dir_all<std::path::PathBuf,std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 444
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1696,"record_offset":157276,"rva":24382464,"va":5393091584}
// 编译实例 RVAs: 0x1740c00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10383056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e8309161ff","size":5,"soft_fail":false,"target_va":5382708864,"va":5393091915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10324353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e87f7662ff","size":5,"soft_fail":false,"target_va":5382768048,"va":5393092396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403107e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21169940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e8ecf8bcfe","size":5,"soft_fail":false,"target_va":5371922400,"va":5393092335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e69f0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18196328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e89858eafe","size":5,"soft_fail":false,"target_va":5374896624,"va":5393092947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18010324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e82c2fedfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393091823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=6
//     target_aliases=5 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10263930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e8866263ff","size":5,"soft_fail":false,"target_va":5382827728,"va":5393091653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141740c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e8a8fdffff","size":5,"soft_fail":false,"target_va":5393091584,"va":5393092179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18755370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e82a2f1e01","size":5,"soft_fail":false,"target_va":5411847168,"va":5393091793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-10408790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e8aa2c61ff","size":5,"soft_fail":false,"target_va":5382683584,"va":5393092369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10382099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e8ed9461ff","size":5,"soft_fail":false,"target_va":5382709584,"va":5393091678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10382554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e8269361ff","size":5,"soft_fail":false,"target_va":5382709552,"va":5393092101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10382474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e8769361ff","size":5,"soft_fail":false,"target_va":5382709520,"va":5393091989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10383055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e8319161ff","size":5,"soft_fail":false,"target_va":5382709008,"va":5393092058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10348056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e8e81962ff","size":5,"soft_fail":false,"target_va":5382744080,"va":5393092131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10321196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393091584,"raw_hex":"e8d48262ff","size":5,"soft_fail":false,"target_va":5382770560,"va":5393091751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1696,"record_offset":160636,"parent_record_offset":157276,"ancestor_record_offset":157276,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1696,"record_offset":158632,"parent_record_offset":157276,"ancestor_record_offset":157276,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1696,"record_offset":158728,"parent_record_offset":157276,"ancestor_record_offset":157276,"inlinee":13008,"source_line":2036,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1696,"record_offset":157476,"parent_record_offset":157276,"ancestor_record_offset":157276,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1696,"record_offset":157756,"parent_record_offset":157276,"ancestor_record_offset":157276,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1696,"record_offset":158980,"parent_record_offset":157276,"ancestor_record_offset":157276,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1696,"record_offset":158812,"parent_record_offset":157276,"ancestor_record_offset":157276,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1696,"record_offset":158252,"parent_record_offset":158228,"ancestor_record_offset":157276,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1696,"record_offset":158228,"parent_record_offset":158172,"ancestor_record_offset":157276,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1696,"record_offset":158200,"parent_record_offset":158172,"ancestor_record_offset":157276,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1696,"record_offset":158172,"parent_record_offset":158148,"ancestor_record_offset":157276,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1696,"record_offset":158148,"parent_record_offset":158124,"ancestor_record_offset":157276,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1696,"record_offset":160744,"parent_record_offset":160708,"ancestor_record_offset":157276,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1696,"record_offset":160708,"parent_record_offset":160684,"ancestor_record_offset":157276,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1696,"record_offset":157528,"parent_record_offset":157476,"ancestor_record_offset":157276,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=83
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1696,"record_offset":158004,"parent_record_offset":157756,"ancestor_record_offset":157276,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1696,"record_offset":160776,"parent_record_offset":160744,"ancestor_record_offset":157276,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1696,"record_offset":157964,"parent_record_offset":157756,"ancestor_record_offset":157276,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1696,"record_offset":160804,"parent_record_offset":160744,"ancestor_record_offset":157276,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1696,"record_offset":160836,"parent_record_offset":160708,"ancestor_record_offset":157276,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1696,"record_offset":157624,"parent_record_offset":157600,"ancestor_record_offset":157276,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1696,"record_offset":157672,"parent_record_offset":157648,"ancestor_record_offset":157276,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1696,"record_offset":157648,"parent_record_offset":157624,"ancestor_record_offset":157276,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1696,"record_offset":157724,"parent_record_offset":157476,"ancestor_record_offset":157276,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1696,"record_offset":157500,"parent_record_offset":157476,"ancestor_record_offset":157276,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1696,"record_offset":157600,"parent_record_offset":157576,"ancestor_record_offset":157276,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1696,"record_offset":157576,"parent_record_offset":157552,"ancestor_record_offset":157276,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$80::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1696,"record_offset":159012,"parent_record_offset":158980,"ancestor_record_offset":157276,"inlinee":5140,"source_line":3338,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1696,"record_offset":157552,"parent_record_offset":157528,"ancestor_record_offset":157276,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1696,"record_offset":158752,"parent_record_offset":158728,"ancestor_record_offset":157276,"inlinee":13010,"source_line":1126,"archive":"_pdb/B/modules/01696.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1696,"record_offset":158776,"parent_record_offset":158752,"ancestor_record_offset":157276,"inlinee":13011,"source_line":1132,"archive":"_pdb/B/modules/01696.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_session.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::startup_session::MaybeExists
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::startup_session::SessionLoadAttempt
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::startup::startup_session::SessionLoadAttempt>", "constructor": "enum2$", "arguments": ["country_core::startup::startup_session::SessionLoadAttempt"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque<enum2$<country_core::startup::startup_session::SessionLoadAttempt>,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque", "arguments": ["enum2$<country_core::startup::startup_session::SessionLoadAttempt>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<enum2$<country_core::startup::startup_session::SessionLoadAttempt>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::vec_deque::VecDeque<enum2$<country_core::startup::startup_session::SessionLoadAttempt>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::impl$1::drop::Dropper<enum2$<country_core::startup::startup_session::SessionLoadAttempt> >", "constructor": "alloc::collections::vec_deque::impl$1::drop::Dropper", "arguments": ["enum2$<country_core::startup::startup_session::SessionLoadAttempt>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::vec_deque::impl$1::drop::Dropper<enum2$<country_core::startup::startup_session::SessionLoadAttempt> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::vec_deque::impl$1::drop::Dropper<enum2$<country_core::startup::startup_session::SessionLoadAttempt> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::startup::startup_session::SessionLoadAttempt> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::startup::startup_session::SessionLoadAttempt>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::grow<enum2$<country_core::startup::startup_session::SessionLoadAttempt>,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::grow", "arguments": ["enum2$<country_core::startup::startup_session::SessionLoadAttempt>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<country_core::startup::startup_session::SessionLoadAttempt>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<country_core::startup::startup_session::SessionLoadAttempt>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> >", "constructor": "core::option::Option", "arguments": ["enum2$<country_core::startup::startup_session::SessionLoadAttempt>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::startup_session::SessionMetaSource
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::startup::startup_session::SessionMetaSource>", "constructor": "enum2$", "arguments": ["country_core::startup::startup_session::SessionMetaSource"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::startup::startup_session::SessionMetaSource> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::startup::startup_session::SessionMetaSource>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_session.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_session.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::writer::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::event::writer::EventWriter<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::session::SessionLoadedCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::session::SessionLoadedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::writer::EventWriter<country_core::resources::window::CursorMovedScreenSpace>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::writer::EventWriter<country_core::resources::window::CursorMovedScreenSpace>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::window::CursorMovedScreenSpace>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::MaybeExists::get
// 信息 ID: 8e865da0984aa59cb2c03c9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::load_game_session; module=1644, inline_record=34428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::SessionLoadAttempt::from_session_id
// 信息 ID: 6634cf708e3d394651291f13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::load_game_session; module=1644, inline_record=27356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::copy_dir_all<std::path::PathBuf,std::path::PathBuf>
// 信息 ID: e66c20e8b24ea2a167112937
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::startup_session::copy_dir_all -> ["std::path::PathBuf", "std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_session::load_game_session @ 0x16710dd -> 0x1740c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_session::copy_dir_all<std::path::PathBuf,std::path::PathBuf> @ 0x1740e53 -> 0x1740c00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_game_session
// 信息 ID: b2130056e1cc6e85384ca731
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9c9fd5aa606df95c65aa8001: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::writer::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::writer::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::writer::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>,bevy_ecs::system::commands::Commands)> > > @ 0x14d5c52 -> 0x166e7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::writer::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::writer::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>,bevy_ecs::system::commands::Commands)> > @ 0x173e8cf -> 0x166e7d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_game_session::closure$0
// 信息 ID: 1e6ef5d174df96196c5b794c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::startup_session::load_game_session", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::load_game_session; module=1644, inline_record=50108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_game_session::closure$1
// 信息 ID: fe909fa7984eb0e543cc6415
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::startup_session::load_game_session", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::startup_session::load_game_session::closure_env$1 <- anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,country_core::startup::startup_session::load_game_session::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,country_core::startup::startup_session::load_game_session::closure_env$1>; module=1629, inline_record=113864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_game_session::closure$2
// 信息 ID: 9bf9e43ef8c1e7858f8c7e72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::startup_session::load_game_session", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::load_game_session; module=1644, inline_record=46032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_session_meta
// 信息 ID: 14e51870ef2523de67745db8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::load_game_session; module=1644, inline_record=48460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
