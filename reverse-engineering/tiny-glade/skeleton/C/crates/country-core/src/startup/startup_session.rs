// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/startup_session.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c61862829b63a1c54167d25b669f18dc739d9c605e27451a46f3ac24be390137
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_session.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1253, 1681, 1712
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
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
// 首条位置: {"module":1681,"record_offset":7424,"rva":25196288,"va":5393905408}
// 编译实例 RVAs: 0x1807700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9219058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e80e5473ff","size":5,"soft_fail":false,"target_va":5384689616,"va":5393908669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9220912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8d04c73ff","size":5,"soft_fail":false,"target_va":5384689312,"va":5393910219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9141849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8a78174ff","size":5,"soft_fail":false,"target_va":5384768304,"va":5393910148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-9250073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8e7da72ff","size":5,"soft_fail":false,"target_va":5384656240,"va":5393906308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9253268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e86cce72ff","size":5,"soft_fail":false,"target_va":5384657024,"va":5393910287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_dir
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9253861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e81bcc72ff","size":5,"soft_fail":false,"target_va":5384656400,"va":5393910256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9254977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8bfc772ff","size":5,"soft_fail":false,"target_va":5384656512,"va":5393911484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9146209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e89f7074ff","size":5,"soft_fail":false,"target_va":5384764464,"va":5393910668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-9296559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8512572ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5393905962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18233810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e82ec6e9fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5393912493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=35
//     evidence={"assembly":"\tcallq\t-18227663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e831dee9fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393906362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25190291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e86da07ffe","size":5,"soft_fail":false,"target_va":5368722192,"va":5393912478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24649105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e86fe287fe","size":5,"soft_fail":false,"target_va":5369270288,"va":5393919388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008d710
//     owner=unresolved_namespace; count=1
//     target_aliases=21 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24626616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8483a88fe","size":5,"soft_fail":false,"target_va":5369288464,"va":5393915075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24584312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e888df88fe","size":5,"soft_fail":false,"target_va":5369335008,"va":5393919315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24570494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8821589fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5393909337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2470
//     owner=unresolved_namespace; count=9
//     target_aliases=4 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24470511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8119c8afe","size":5,"soft_fail":false,"target_va":5369439344,"va":5393909850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404378e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20781322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8f6e6c2fe","size":5,"soft_fail":false,"target_va":5373130976,"va":5393912293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15323544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8682e16ff","size":5,"soft_fail":false,"target_va":5378594656,"va":5393918195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409713b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15302224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8b08116ff","size":5,"soft_fail":false,"target_va":5378610096,"va":5393912315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409784f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15275279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8f1ea16ff","size":5,"soft_fail":false,"target_va":5378639088,"va":5393914362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099a4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15136186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8460a19ff","size":5,"soft_fail":false,"target_va":5378778304,"va":5393914485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099cef0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15126792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8f82e19ff","size":5,"soft_fail":false,"target_va":5378789104,"va":5393915891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099d790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15121941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8eb4119ff","size":5,"soft_fail":false,"target_va":5378791312,"va":5393913248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a78b0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15081741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8f3de19ff","size":5,"soft_fail":false,"target_va":5378832560,"va":5393914296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aafe00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13997001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8376c2aff","size":5,"soft_fail":false,"target_va":5379915264,"va":5393912260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ab0bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13996431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8716e2aff","size":5,"soft_fail":false,"target_va":5379918784,"va":5393915210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ab1020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13987475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e86d912aff","size":5,"soft_fail":false,"target_va":5379919904,"va":5393907374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b2afc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13498901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8eb0532ff","size":5,"soft_fail":false,"target_va":5380419520,"va":5393918416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b0460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6653111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8497b9aff","size":5,"soft_fail":false,"target_va":5387256928,"va":5393910034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14152c940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3000102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8da38d2ff","size":5,"soft_fail":false,"target_va":5390911808,"va":5393911905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162c470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1955125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8cb2ae2ff","size":5,"soft_fail":false,"target_va":5391959152,"va":5393914272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141979720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1514627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8831c1700","size":5,"soft_fail":false,"target_va":5395420960,"va":5393906328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419b9d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1771142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e886061b00","size":5,"soft_fail":false,"target_va":5395684656,"va":5393913509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22437543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8a75e5601","size":5,"soft_fail":false,"target_va":5416350384,"va":5393912836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22444710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8a67a5601","size":5,"soft_fail":false,"target_va":5416350720,"va":5393906005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22438261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e875615601","size":5,"soft_fail":false,"target_va":5416353040,"va":5393914774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=26
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23151484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e87c436101","size":5,"soft_fail":false,"target_va":5417057056,"va":5393905567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22705919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8ff765a01","size":5,"soft_fail":false,"target_va":5416620769,"va":5393914845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22457692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e85cad5601","size":5,"soft_fail":false,"target_va":5416368112,"va":5393910415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-11787711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e841224cff","size":5,"soft_fail":false,"target_va":5382122800,"va":5393910506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9282515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e82d5c72ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5393906590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22706642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8d2795a01","size":5,"soft_fail":false,"target_va":5416621536,"va":5393914889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22707034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e85a7b5a01","size":5,"soft_fail":false,"target_va":5416621600,"va":5393914561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22707393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8c17c5a01","size":5,"soft_fail":false,"target_va":5416622208,"va":5393914810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22707580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e87c7d5a01","size":5,"soft_fail":false,"target_va":5416622560,"va":5393914975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22707282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8527c5a01","size":5,"soft_fail":false,"target_va":5416622240,"va":5393914953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<country_core::startup::startup_session::SessionLoadAttempt> > > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t17250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e862430000","size":5,"soft_fail":false,"target_va":5393926992,"va":5393909737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6993196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8d44a95ff","size":5,"soft_fail":false,"target_va":5386926048,"va":5393919239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::save::load_game::PostGameLoadCommand> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8182a0000","size":5,"soft_fail":false,"target_va":5393927504,"va":5393916723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t22707010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8427b5a01","size":5,"soft_fail":false,"target_va":5416621632,"va":5393914617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5923633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e831635a00","size":5,"soft_fail":false,"target_va":5399835568,"va":5393911930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9080247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8497275ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5393910322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::glade_settings::GladeSettings::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13652825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8a7ac2fff","size":5,"soft_fail":false,"target_va":5380261184,"va":5393914004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13638954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8d6e22fff","size":5,"soft_fail":false,"target_va":5380267312,"va":5393906261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::startup_session::resolve_next_session
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e86f390000","size":5,"soft_fail":false,"target_va":5393920400,"va":5393905692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_completion_checker::clear_session_loading_marker
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13460057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8a79d32ff","size":5,"soft_fail":false,"target_va":5380449552,"va":5393909604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::metadata
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9183287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8c9df73ff","size":5,"soft_fail":false,"target_va":5384724912,"va":5393908194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9183971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e81ddd73ff","size":5,"soft_fail":false,"target_va":5384725152,"va":5393909118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9849647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393905408,"raw_hex":"e8d1b469ff","size":5,"soft_fail":false,"target_va":5384058928,"va":5393908570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1681,"record_offset":23584,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1681,"record_offset":23400,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1681,"record_offset":8116,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":11703,"source_line":781,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1681,"record_offset":15744,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":12109,"source_line":2064,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1681,"record_offset":11528,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":5253,"source_line":2205,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1681,"record_offset":19728,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":11475,"source_line":131,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":19700,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":16976,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":17356,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":19916,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1681,"record_offset":22472,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1681,"record_offset":7996,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1681,"record_offset":8396,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1681,"record_offset":11128,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":11211,"source_line":90,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::remove_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1681,"record_offset":7732,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":11390,"source_line":861,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1681,"record_offset":17152,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1681,"record_offset":8228,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1681,"record_offset":19976,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":6943,"source_line":384,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1681,"record_offset":8056,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1681,"record_offset":19888,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":22380,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":25056,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":22228,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":12408,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":17032,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":9800,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1681,"record_offset":14120,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1681,"record_offset":16904,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::meta_commander::MetaCommandFence::unblock
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/meta_commander.rs
//     evidence={"module":1681,"record_offset":22708,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":32502,"source_line":120,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::meta_commander::MetaCommander::extend_script
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/meta_commander.rs
//     evidence={"module":1681,"record_offset":22680,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":28963,"source_line":150,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1681,"record_offset":25620,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":35097,"source_line":286,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::SessionId::snapshots_folder_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1681,"record_offset":10080,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":35109,"source_line":146,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1681,"record_offset":25420,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":27997,"source_line":208,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::SessionMeta::write_to_disk
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1681,"record_offset":8752,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":34501,"source_line":212,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_session::MaybeExists::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1681,"record_offset":12488,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":82748,"source_line":37,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1681,"record_offset":8284,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1681,"record_offset":8144,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1681,"record_offset":17068,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1681,"record_offset":13292,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1681,"record_offset":15420,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1681,"record_offset":22096,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":14220,"source_line":3443,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1681,"record_offset":11952,"parent_record_offset":7424,"ancestor_record_offset":7424,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":23484,"parent_record_offset":23460,"ancestor_record_offset":7424,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":23508,"parent_record_offset":23484,"ancestor_record_offset":7424,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":9056,"parent_record_offset":9032,"ancestor_record_offset":7424,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":9080,"parent_record_offset":9056,"ancestor_record_offset":7424,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":23532,"parent_record_offset":23508,"ancestor_record_offset":7424,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":9104,"parent_record_offset":9080,"ancestor_record_offset":7424,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":23460,"parent_record_offset":23428,"ancestor_record_offset":7424,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":9032,"parent_record_offset":8976,"ancestor_record_offset":7424,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1681,"record_offset":23428,"parent_record_offset":23400,"ancestor_record_offset":7424,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1681,"record_offset":26240,"parent_record_offset":26212,"ancestor_record_offset":7424,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1681,"record_offset":15980,"parent_record_offset":15744,"ancestor_record_offset":7424,"inlinee":12111,"source_line":208,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1681,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":7424,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1681,"record_offset":15788,"parent_record_offset":15744,"ancestor_record_offset":7424,"inlinee":12110,"source_line":1650,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1681,"record_offset":11600,"parent_record_offset":11556,"ancestor_record_offset":7424,"inlinee":5255,"source_line":236,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1681,"record_offset":16008,"parent_record_offset":15980,"ancestor_record_offset":7424,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::push_back_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1681,"record_offset":11556,"parent_record_offset":11528,"ancestor_record_offset":7424,"inlinee":5254,"source_line":2223,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1681,"record_offset":11712,"parent_record_offset":11556,"ancestor_record_offset":7424,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1681,"record_offset":11748,"parent_record_offset":11712,"ancestor_record_offset":7424,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":11648,"parent_record_offset":11624,"ancestor_record_offset":7424,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":7424,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":7848,"parent_record_offset":7820,"ancestor_record_offset":7424,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":7932,"parent_record_offset":7908,"ancestor_record_offset":7424,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":9004,"parent_record_offset":8976,"ancestor_record_offset":7424,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":8976,"parent_record_offset":8952,"ancestor_record_offset":7424,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":7908,"parent_record_offset":7876,"ancestor_record_offset":7424,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":8664,"parent_record_offset":8640,"ancestor_record_offset":7424,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":7424,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":7876,"parent_record_offset":7848,"ancestor_record_offset":7424,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":8952,"parent_record_offset":8928,"ancestor_record_offset":7424,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1681,"record_offset":23608,"parent_record_offset":23584,"ancestor_record_offset":7424,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1681,"record_offset":19756,"parent_record_offset":19728,"ancestor_record_offset":7424,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1681,"record_offset":18060,"parent_record_offset":18024,"ancestor_record_offset":7424,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1681,"record_offset":18024,"parent_record_offset":18000,"ancestor_record_offset":7424,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":8592,"parent_record_offset":8552,"ancestor_record_offset":7424,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":23632,"parent_record_offset":23608,"ancestor_record_offset":7424,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":8520,"parent_record_offset":8484,"ancestor_record_offset":7424,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":8552,"parent_record_offset":8520,"ancestor_record_offset":7424,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":7820,"parent_record_offset":7788,"ancestor_record_offset":7424,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1681,"record_offset":20660,"parent_record_offset":20632,"ancestor_record_offset":7424,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1681,"record_offset":21148,"parent_record_offset":21124,"ancestor_record_offset":7424,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1681,"record_offset":20008,"parent_record_offset":19976,"ancestor_record_offset":7424,"inlinee":10931,"source_line":244,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1681,"record_offset":20476,"parent_record_offset":19976,"ancestor_record_offset":7424,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1681,"record_offset":8024,"parent_record_offset":7996,"ancestor_record_offset":7424,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1681,"record_offset":8420,"parent_record_offset":8396,"ancestor_record_offset":7424,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1681,"record_offset":11236,"parent_record_offset":11212,"ancestor_record_offset":7424,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1681,"record_offset":11212,"parent_record_offset":11128,"ancestor_record_offset":7424,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1681,"record_offset":8484,"parent_record_offset":8420,"ancestor_record_offset":7424,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1681,"record_offset":7760,"parent_record_offset":7732,"ancestor_record_offset":7424,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1681,"record_offset":7788,"parent_record_offset":7760,"ancestor_record_offset":7424,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1681,"record_offset":17208,"parent_record_offset":17180,"ancestor_record_offset":7424,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1681,"record_offset":19820,"parent_record_offset":19784,"ancestor_record_offset":7424,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1681,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":7424,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1681,"record_offset":21276,"parent_record_offset":21248,"ancestor_record_offset":7424,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1681,"record_offset":15872,"parent_record_offset":15844,"ancestor_record_offset":7424,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":25080,"parent_record_offset":25056,"ancestor_record_offset":7424,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":8252,"parent_record_offset":8228,"ancestor_record_offset":7424,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":17180,"parent_record_offset":17152,"ancestor_record_offset":7424,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=202
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":8808,"parent_record_offset":8752,"ancestor_record_offset":7424,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1681,"record_offset":11884,"parent_record_offset":11712,"ancestor_record_offset":7424,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1681,"record_offset":20504,"parent_record_offset":20476,"ancestor_record_offset":7424,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1681,"record_offset":18092,"parent_record_offset":18060,"ancestor_record_offset":7424,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1681,"record_offset":20532,"parent_record_offset":20476,"ancestor_record_offset":7424,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1681,"record_offset":22504,"parent_record_offset":22472,"ancestor_record_offset":7424,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":20556,"parent_record_offset":20532,"ancestor_record_offset":7424,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":8704,"parent_record_offset":8552,"ancestor_record_offset":7424,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1681,"record_offset":12592,"parent_record_offset":12564,"ancestor_record_offset":7424,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1681,"record_offset":17280,"parent_record_offset":17256,"ancestor_record_offset":7424,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1681,"record_offset":17304,"parent_record_offset":17280,"ancestor_record_offset":7424,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1681,"record_offset":19784,"parent_record_offset":19756,"ancestor_record_offset":7424,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1681,"record_offset":17232,"parent_record_offset":17208,"ancestor_record_offset":7424,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1681,"record_offset":22732,"parent_record_offset":22708,"ancestor_record_offset":7424,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1681,"record_offset":22876,"parent_record_offset":22840,"ancestor_record_offset":7424,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1681,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":7424,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1681,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":7424,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1681,"record_offset":22756,"parent_record_offset":22732,"ancestor_record_offset":7424,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1681,"record_offset":18120,"parent_record_offset":18060,"ancestor_record_offset":7424,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1681,"record_offset":18152,"parent_record_offset":18024,"ancestor_record_offset":7424,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1681,"record_offset":17116,"parent_record_offset":17092,"ancestor_record_offset":7424,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1681,"record_offset":25444,"parent_record_offset":25420,"ancestor_record_offset":7424,"inlinee":27998,"source_line":52,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1681,"record_offset":25136,"parent_record_offset":25112,"ancestor_record_offset":7424,"inlinee":11529,"source_line":35,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_session::load_game_session::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1681,"record_offset":22260,"parent_record_offset":22228,"ancestor_record_offset":7424,"inlinee":82749,"source_line":485,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1681,"record_offset":8332,"parent_record_offset":8308,"ancestor_record_offset":7424,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1681,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":7424,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1681,"record_offset":8356,"parent_record_offset":8332,"ancestor_record_offset":7424,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1681,"record_offset":13352,"parent_record_offset":13292,"ancestor_record_offset":7424,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1681,"record_offset":13324,"parent_record_offset":13292,"ancestor_record_offset":7424,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1681,"record_offset":12564,"parent_record_offset":12536,"ancestor_record_offset":7424,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1681,"record_offset":12536,"parent_record_offset":12488,"ancestor_record_offset":7424,"inlinee":14220,"source_line":3443,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1681,"record_offset":8780,"parent_record_offset":8752,"ancestor_record_offset":7424,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1681,"record_offset":17092,"parent_record_offset":17068,"ancestor_record_offset":7424,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::MaybeExists::get
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1433C
// 首条位置: {"module":1681,"inlinee":82748}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":12668,"parent_record_offset":12488,"ancestor_record_offset":12488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1681,"record_offset":12536,"parent_record_offset":12488,"ancestor_record_offset":12488,"inlinee":14220,"source_line":3443,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":12916,"parent_record_offset":12892,"ancestor_record_offset":12488,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":12940,"parent_record_offset":12916,"ancestor_record_offset":12488,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":12964,"parent_record_offset":12940,"ancestor_record_offset":12488,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":12892,"parent_record_offset":12836,"ancestor_record_offset":12488,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":12864,"parent_record_offset":12836,"ancestor_record_offset":12488,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":12836,"parent_record_offset":12812,"ancestor_record_offset":12488,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":12812,"parent_record_offset":12788,"ancestor_record_offset":12488,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":12632,"parent_record_offset":12536,"ancestor_record_offset":12488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1681,"record_offset":12592,"parent_record_offset":12564,"ancestor_record_offset":12488,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1681,"record_offset":12564,"parent_record_offset":12536,"ancestor_record_offset":12488,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::resolve_next_session
// 记录形式: compiled_procedure; instances=1
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1681,"record_offset":32816,"rva":25211280,"va":5393920400}
// 编译实例 RVAs: 0x180b190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9125764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e87cc074ff","size":5,"soft_fail":false,"target_va":5384795984,"va":5393921743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9311357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e883eb71ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5393920760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18242691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e87da3e9fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5393921374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-18242868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e8cca2e9fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393921567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25199169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e8bf7d7ffe","size":5,"soft_fail":false,"target_va":5368722192,"va":5393921356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24491000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e8084c8afe","size":5,"soft_fail":false,"target_va":5369431360,"va":5393922355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2470
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24482832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e8f06b8afe","size":5,"soft_fail":false,"target_va":5369439344,"va":5393922171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f7ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2177150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e882c7deff","size":5,"soft_fail":false,"target_va":5391744752,"va":5393921897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23136166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e8a6076101","size":5,"soft_fail":false,"target_va":5417057056,"va":5393920885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22699456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e8c05d5a01","size":5,"soft_fail":false,"target_va":5416620794,"va":5393921333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9296783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e8712472ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5393920858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22699892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e8745f5a01","size":5,"soft_fail":false,"target_va":5416622208,"va":5393922311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::startup_session::load_session_meta
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e85ffeffff","size":5,"soft_fail":false,"target_va":5393920112,"va":5393920524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_completion_checker::is_session_ok_to_load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13472472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920400,"raw_hex":"e8286d32ff","size":5,"soft_fail":false,"target_va":5380448448,"va":5393920915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":33700,"parent_record_offset":32816,"ancestor_record_offset":32816,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":33420,"parent_record_offset":32816,"ancestor_record_offset":32816,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1681,"record_offset":33532,"parent_record_offset":32816,"ancestor_record_offset":32816,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1681,"record_offset":34308,"parent_record_offset":32816,"ancestor_record_offset":32816,"inlinee":36640,"source_line":619,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1681,"record_offset":32996,"parent_record_offset":32816,"ancestor_record_offset":32816,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1681,"record_offset":34280,"parent_record_offset":32816,"ancestor_record_offset":32816,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1681,"record_offset":34464,"parent_record_offset":32816,"ancestor_record_offset":32816,"inlinee":8466,"source_line":176,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$4::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1681,"record_offset":36124,"parent_record_offset":32816,"ancestor_record_offset":32816,"inlinee":35130,"source_line":262,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1681,"record_offset":33588,"parent_record_offset":32816,"ancestor_record_offset":32816,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1681,"record_offset":33448,"parent_record_offset":32816,"ancestor_record_offset":32816,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":34996,"parent_record_offset":34972,"ancestor_record_offset":32816,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":35020,"parent_record_offset":34996,"ancestor_record_offset":32816,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":34160,"parent_record_offset":34136,"ancestor_record_offset":32816,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":34184,"parent_record_offset":34160,"ancestor_record_offset":32816,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":35044,"parent_record_offset":35020,"ancestor_record_offset":32816,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":34208,"parent_record_offset":34184,"ancestor_record_offset":32816,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":34972,"parent_record_offset":34812,"ancestor_record_offset":32816,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":34136,"parent_record_offset":34072,"ancestor_record_offset":32816,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":33800,"parent_record_offset":33772,"ancestor_record_offset":32816,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":34740,"parent_record_offset":34708,"ancestor_record_offset":32816,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":34104,"parent_record_offset":34072,"ancestor_record_offset":32816,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":34072,"parent_record_offset":34044,"ancestor_record_offset":32816,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":33856,"parent_record_offset":33828,"ancestor_record_offset":32816,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":33828,"parent_record_offset":33800,"ancestor_record_offset":32816,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":34812,"parent_record_offset":34772,"ancestor_record_offset":32816,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":34772,"parent_record_offset":34740,"ancestor_record_offset":32816,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":34044,"parent_record_offset":34016,"ancestor_record_offset":32816,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":34860,"parent_record_offset":34812,"ancestor_record_offset":32816,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1681,"record_offset":34648,"parent_record_offset":34572,"ancestor_record_offset":32816,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":33772,"parent_record_offset":33744,"ancestor_record_offset":32816,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":33744,"parent_record_offset":33700,"ancestor_record_offset":32816,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":34620,"parent_record_offset":34572,"ancestor_record_offset":32816,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":34424,"parent_record_offset":34400,"ancestor_record_offset":32816,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":34708,"parent_record_offset":34648,"ancestor_record_offset":32816,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":34532,"parent_record_offset":34464,"ancestor_record_offset":32816,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":34572,"parent_record_offset":34532,"ancestor_record_offset":32816,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1681,"record_offset":34884,"parent_record_offset":34860,"ancestor_record_offset":32816,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1681,"record_offset":33160,"parent_record_offset":33036,"ancestor_record_offset":32816,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1681,"record_offset":34932,"parent_record_offset":34908,"ancestor_record_offset":32816,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1681,"record_offset":35144,"parent_record_offset":35100,"ancestor_record_offset":32816,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":1681,"record_offset":35100,"parent_record_offset":34648,"ancestor_record_offset":32816,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1681,"record_offset":35236,"parent_record_offset":34648,"ancestor_record_offset":32816,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1681,"record_offset":34908,"parent_record_offset":34884,"ancestor_record_offset":32816,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":33184,"parent_record_offset":33160,"ancestor_record_offset":32816,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":35280,"parent_record_offset":34464,"ancestor_record_offset":32816,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":33356,"parent_record_offset":33036,"ancestor_record_offset":32816,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1681,"record_offset":35196,"parent_record_offset":35172,"ancestor_record_offset":32816,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1681,"record_offset":35172,"parent_record_offset":35144,"ancestor_record_offset":32816,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1681,"record_offset":33100,"parent_record_offset":33076,"ancestor_record_offset":32816,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1681,"record_offset":33124,"parent_record_offset":33100,"ancestor_record_offset":32816,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":1681,"record_offset":35344,"parent_record_offset":35308,"ancestor_record_offset":32816,"inlinee":8473,"source_line":24,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":1681,"record_offset":35308,"parent_record_offset":35280,"ancestor_record_offset":32816,"inlinee":8471,"source_line":27,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":1681,"record_offset":35388,"parent_record_offset":35308,"ancestor_record_offset":32816,"inlinee":8475,"source_line":11,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1681,"record_offset":35488,"parent_record_offset":35456,"ancestor_record_offset":32816,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1681,"record_offset":33328,"parent_record_offset":33036,"ancestor_record_offset":32816,"inlinee":35130,"source_line":262,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":1681,"record_offset":35456,"parent_record_offset":35424,"ancestor_record_offset":32816,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_session::resolve_next_session::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1681,"record_offset":34400,"parent_record_offset":34308,"ancestor_record_offset":32816,"inlinee":82752,"source_line":147,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::startup_session::resolve_next_session::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_session.rs
//     evidence={"module":1681,"record_offset":33036,"parent_record_offset":32996,"ancestor_record_offset":32816,"inlinee":82751,"source_line":176,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1681,"record_offset":33264,"parent_record_offset":33240,"ancestor_record_offset":32816,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1681,"record_offset":33216,"parent_record_offset":33036,"ancestor_record_offset":32816,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1681,"record_offset":33240,"parent_record_offset":33216,"ancestor_record_offset":32816,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1681,"record_offset":33288,"parent_record_offset":33264,"ancestor_record_offset":32816,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1681,"record_offset":33076,"parent_record_offset":33036,"ancestor_record_offset":32816,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::resolve_next_session::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14340
// 首条位置: {"module":1681,"inlinee":82752}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":34424,"parent_record_offset":34400,"ancestor_record_offset":34400,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::resolve_next_session::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1433F
// 首条位置: {"module":1681,"inlinee":82751}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1681,"record_offset":33160,"parent_record_offset":33036,"ancestor_record_offset":33036,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":33356,"parent_record_offset":33036,"ancestor_record_offset":33036,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1681,"record_offset":33328,"parent_record_offset":33036,"ancestor_record_offset":33036,"inlinee":35130,"source_line":262,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1681,"record_offset":33216,"parent_record_offset":33036,"ancestor_record_offset":33036,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1681,"record_offset":33076,"parent_record_offset":33036,"ancestor_record_offset":33036,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":33184,"parent_record_offset":33160,"ancestor_record_offset":33036,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1681,"record_offset":33100,"parent_record_offset":33076,"ancestor_record_offset":33036,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1681,"record_offset":33124,"parent_record_offset":33100,"ancestor_record_offset":33036,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1681,"record_offset":33264,"parent_record_offset":33240,"ancestor_record_offset":33036,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1681,"record_offset":33240,"parent_record_offset":33216,"ancestor_record_offset":33036,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1681,"record_offset":33288,"parent_record_offset":33264,"ancestor_record_offset":33036,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_game_session::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 444
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112DC
// 首条位置: {"module":1253,"inlinee":70364}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":39528,"parent_record_offset":39508,"ancestor_record_offset":39508,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":39576,"parent_record_offset":39552,"ancestor_record_offset":39508,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1253,"record_offset":39552,"parent_record_offset":39528,"ancestor_record_offset":39508,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_game_session::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 485
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1433D
// 首条位置: {"module":1681,"inlinee":82749}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1681,"record_offset":22284,"parent_record_offset":22260,"ancestor_record_offset":22260,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::load_session_meta
// 记录形式: compiled_procedure; instances=1
// 源码行: 553
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1681,"record_offset":31220,"rva":25210992,"va":5393920112}
// 编译实例 RVAs: 0x180b070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9263946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920112,"raw_hex":"e8b6a472ff","size":5,"soft_fail":false,"target_va":5384656240,"va":5393920181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-18241528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920112,"raw_hex":"e808a8e9fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393920227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13652837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920112,"raw_hex":"e89bac2fff","size":5,"soft_fail":false,"target_va":5380267312,"va":5393920144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::try_load_from_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13657327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393920112,"raw_hex":"e8119b2fff","size":5,"soft_fail":false,"target_va":5380262880,"va":5393920202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":31396,"parent_record_offset":31220,"ancestor_record_offset":31220,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1681,"record_offset":31368,"parent_record_offset":31220,"ancestor_record_offset":31220,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":31616,"parent_record_offset":31592,"ancestor_record_offset":31220,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":31640,"parent_record_offset":31616,"ancestor_record_offset":31220,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":31664,"parent_record_offset":31640,"ancestor_record_offset":31220,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":31592,"parent_record_offset":31540,"ancestor_record_offset":31220,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":31564,"parent_record_offset":31540,"ancestor_record_offset":31220,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":31540,"parent_record_offset":31516,"ancestor_record_offset":31220,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":31516,"parent_record_offset":31496,"ancestor_record_offset":31220,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":31416,"parent_record_offset":31396,"ancestor_record_offset":31220,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_session::copy_dir_all<std::path::PathBuf,std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 559
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1712,"record_offset":136740,"rva":26975536,"va":5395684656}
// 编译实例 RVAs: 0x19b9d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10995139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e83d3a58ff","size":5,"soft_fail":false,"target_va":5384689616,"va":5395684750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::file_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10995589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e87b3858ff","size":5,"soft_fail":false,"target_va":5384689584,"va":5395685168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10995838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e8823758ff","size":5,"soft_fail":false,"target_va":5384689312,"va":5395685145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10916735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e8816c59ff","size":5,"soft_fail":false,"target_va":5384768304,"va":5395685034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11028978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e80eb657ff","size":5,"soft_fail":false,"target_va":5384656240,"va":5395685213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10921115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e8655b59ff","size":5,"soft_fail":false,"target_va":5384764464,"va":5395685574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=17
//     evidence={"assembly":"\tcallq\t-20006093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e833bbcefe","size":5,"soft_fail":false,"target_va":5375678704,"va":5395684792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26351048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e838ea6dfe","size":5,"soft_fail":false,"target_va":5369335008,"va":5395686051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419b9d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e89dfdffff","size":5,"soft_fail":false,"target_va":5395684656,"va":5395685262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: startup_session.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21372138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e8ea1c4601","size":5,"soft_fail":false,"target_va":5417057056,"va":5395684913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-13562751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e8810c31ff","size":5,"soft_fail":false,"target_va":5382122800,"va":5395685546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10961725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e8c3bc58ff","size":5,"soft_fail":false,"target_va":5384723728,"va":5395685448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10959723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395684656,"raw_hex":"e895c458ff","size":5,"soft_fail":false,"target_va":5384725152,"va":5395684870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1712,"record_offset":137056,"parent_record_offset":136740,"ancestor_record_offset":136740,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1712,"record_offset":138584,"parent_record_offset":136740,"ancestor_record_offset":136740,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1712,"record_offset":138648,"parent_record_offset":136740,"ancestor_record_offset":136740,"inlinee":13254,"source_line":2610,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1712,"record_offset":138736,"parent_record_offset":136740,"ancestor_record_offset":136740,"inlinee":13260,"source_line":2392,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1712,"record_offset":140224,"parent_record_offset":136740,"ancestor_record_offset":136740,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1712,"record_offset":136944,"parent_record_offset":136740,"ancestor_record_offset":136740,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1712,"record_offset":137764,"parent_record_offset":136740,"ancestor_record_offset":136740,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1712,"record_offset":138792,"parent_record_offset":136740,"ancestor_record_offset":136740,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":137276,"parent_record_offset":137252,"ancestor_record_offset":136740,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":137300,"parent_record_offset":137276,"ancestor_record_offset":136740,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":137324,"parent_record_offset":137300,"ancestor_record_offset":136740,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":137252,"parent_record_offset":137200,"ancestor_record_offset":136740,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1712,"record_offset":137224,"parent_record_offset":137200,"ancestor_record_offset":136740,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1712,"record_offset":137200,"parent_record_offset":137176,"ancestor_record_offset":136740,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1712,"record_offset":137176,"parent_record_offset":137156,"ancestor_record_offset":136740,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1712,"record_offset":141840,"parent_record_offset":141804,"ancestor_record_offset":136740,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1712,"record_offset":141804,"parent_record_offset":141780,"ancestor_record_offset":136740,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1712,"record_offset":136996,"parent_record_offset":136944,"ancestor_record_offset":136740,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=112
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1712,"record_offset":137076,"parent_record_offset":137056,"ancestor_record_offset":136740,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1712,"record_offset":141872,"parent_record_offset":141840,"ancestor_record_offset":136740,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1712,"record_offset":137804,"parent_record_offset":137764,"ancestor_record_offset":136740,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1712,"record_offset":141900,"parent_record_offset":141840,"ancestor_record_offset":136740,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1712,"record_offset":141932,"parent_record_offset":141804,"ancestor_record_offset":136740,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1712,"record_offset":137024,"parent_record_offset":136944,"ancestor_record_offset":136740,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1712,"record_offset":136968,"parent_record_offset":136944,"ancestor_record_offset":136740,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1712,"record_offset":138672,"parent_record_offset":138648,"ancestor_record_offset":136740,"inlinee":13256,"source_line":181,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1712,"record_offset":138760,"parent_record_offset":138736,"ancestor_record_offset":136740,"inlinee":13261,"source_line":1196,"archive":"_pdb/C/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1712,"record_offset":138696,"parent_record_offset":138672,"ancestor_record_offset":136740,"inlinee":13258,"source_line":1187,"archive":"_pdb/C/modules/01712.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::SessionLoadedCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::SessionLoadedCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>,bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>,bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiPointerCapture>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<enum2$<bevy_input::mouse::MouseButton>,std::hash::random::RandomState,alloc::alloc::Global> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>", "bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiPointerCapture>", "bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<enum2$<bevy_input::mouse::MouseButton>,std::hash::random::RandomState,alloc::alloc::Global> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiPointerCapture>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<enum2$<bevy_input::mouse::MouseButton>,std::hash::random::RandomState,alloc::alloc::Global> >

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::MaybeExists::get
// 信息 ID: c2d9d94a501c33589372dc49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::load_game_session; module=1681, inline_record=12488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::copy_dir_all<std::path::PathBuf,std::path::PathBuf>
// 信息 ID: 2fb7f5ca61dc60232c4abe87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::startup_session::copy_dir_all -> ["std::path::PathBuf", "std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_session::load_game_session @ 0x18096a5 -> 0x19b9d30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_session::copy_dir_all<std::path::PathBuf,std::path::PathBuf> @ 0x19b9f8e -> 0x19b9d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_game_session
// 信息 ID: 12730e53aa71b61f48ea4822
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8d0f9d77b5e4798b339a05b7: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>,bevy_ecs::system::commands::Commands)> > @ 0x1917f79 -> 0x1807700 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>,bevy_ecs::system::commands::Commands)> @ 0x19b902c -> 0x1807700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_game_session::closure$0
// 信息 ID: c1a632b9a6a66d457d19483b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::startup_session::load_game_session", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::startup_session::load_game_session::closure_env$0 <- anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,country_core::startup::startup_session::load_game_session::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,country_core::startup::startup_session::load_game_session::closure_env$0>; module=1253, inline_record=39508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_game_session::closure$1
// 信息 ID: 3bca79d591aa1c710ce7865b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::startup_session::load_game_session", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::load_game_session; module=1681, inline_record=22260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::load_session_meta
// 信息 ID: e2c421f2494cebd7fcdcbefc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_session::resolve_next_session @ 0x180b20c -> 0x180b070 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::resolve_next_session
// 信息 ID: 75df5edcabaede63d95f3172
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::swap_replacement_mods::swap_replacement_mods @ 0x908331 -> 0x180b190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::swap_replacement_mods::preactivate_boot_replacement_mods @ 0x908e28 -> 0x180b190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_session::load_game_session @ 0x180781c -> 0x180b190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::resolve_next_session::closure$0
// 信息 ID: c02da0c7243f22d148db7765
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::startup_session::resolve_next_session", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::resolve_next_session; module=1681, inline_record=34400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_session::resolve_next_session::closure$1
// 信息 ID: e0f13c358402acc1185625a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::startup_session::resolve_next_session", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_session::resolve_next_session; module=1681, inline_record=33036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
