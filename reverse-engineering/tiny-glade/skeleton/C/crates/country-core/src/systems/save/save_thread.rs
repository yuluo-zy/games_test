// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/save/save_thread.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e6ad3664aa38674b73f9e56fec5cc799218a2bcc702020b2c6ae33b0ca579cf9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: save_thread.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 528, 535, 558, 1253, 1643, 1658, 1662, 1800, 3393
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bitvec; // namespace candidate
//   package identities: [{"name":"bitvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.15.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::execute_save_cmd
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1800,"record_offset":35936,"rva":28773024,"va":5397482144}
// 编译实例 RVAs: 0x1b70aa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::lifecycle::Packet<()>>>::drop_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12792548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e81ccd3cff","size":5,"soft_fail":false,"target_va":5384703584,"va":5397496127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12792188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e884ce3cff","size":5,"soft_fail":false,"target_va":5384703904,"va":5397496087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-12828823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8693f3cff","size":5,"soft_fail":false,"target_va":5384659264,"va":5397488082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12799521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8dfb13cff","size":5,"soft_fail":false,"target_va":5384689616,"va":5397489132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12804127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8e19f3cff","size":5,"soft_fail":false,"target_va":5384689312,"va":5397493434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12724699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e825d63dff","size":5,"soft_fail":false,"target_va":5384768304,"va":5397492998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12850239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8c1eb3bff","size":5,"soft_fail":false,"target_va":5384644912,"va":5397495146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-12828743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8b93f3cff","size":5,"soft_fail":false,"target_va":5384656240,"va":5397484978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::components
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12834551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e809293cff","size":5,"soft_fail":false,"target_va":5384653152,"va":5397487698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::extension
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12836561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e82f213cff","size":5,"soft_fail":false,"target_va":5384656912,"va":5397493468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12842243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8fd0a3cff","size":5,"soft_fail":false,"target_va":5384657024,"va":5397499262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_file
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12832285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8e3313cff","size":5,"soft_fail":false,"target_va":5384656800,"va":5397489080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12728578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8fec63dff","size":5,"soft_fail":false,"target_va":5384764464,"va":5397493037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::thread::builder::Builder>::name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12846505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e857fa3bff","size":5,"soft_fail":false,"target_va":5384649296,"va":5397495796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant as core::ops::arith::Add<core::time::Duration>>::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12727933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e883c93dff","size":5,"soft_fail":false,"target_va":5384768864,"va":5397496792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12841719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8090d3cff","size":5,"soft_fail":false,"target_va":5384648032,"va":5397489746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-12873118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e862923bff","size":5,"soft_fail":false,"target_va":5384609408,"va":5397482521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21816891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8c519b3fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5397495574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=82
//     evidence={"assembly":"\tcallq\t-21806310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e81a43b3fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397485009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28773372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e804f448fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5397495559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007db70
//     owner=unresolved_namespace; count=2
//     target_aliases=27 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28282338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e81e7250fe","size":5,"soft_fail":false,"target_va":5369224048,"va":5397506381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140083cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=131 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28261850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e826c250fe","size":5,"soft_fail":false,"target_va":5369248944,"va":5397510789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=4
//     target_aliases=37 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28236513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e81f2551fe","size":5,"soft_fail":false,"target_va":5369270288,"va":5397506796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28214538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8f67a51fe","size":5,"soft_fail":false,"target_va":5369305584,"va":5397520117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28159143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8595352fe","size":5,"soft_fail":false,"target_va":5369335008,"va":5397494146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=11
//     target_aliases=3 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28153967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8916752fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5397492810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28167667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e80d3252fe","size":5,"soft_fail":false,"target_va":5369343504,"va":5397511166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009f5b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28138532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8dca352fe","size":5,"soft_fail":false,"target_va":5369361840,"va":5397500367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040ebd0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24537464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8889689fe","size":5,"soft_fail":false,"target_va":5372963792,"va":5397501251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404378e0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24369750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8aa258cfe","size":5,"soft_fail":false,"target_va":5373130976,"va":5397500721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23393047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8e90c9bfe","size":5,"soft_fail":false,"target_va":5374098704,"va":5397491746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140680730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21973778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8eeb4b0fe","size":5,"soft_fail":false,"target_va":5375526704,"va":5397500477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14091bd20
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19226632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8f89fdafe","size":5,"soft_fail":false,"target_va":5378260256,"va":5397486883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18914275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e81d64dffe","size":5,"soft_fail":false,"target_va":5378595264,"va":5397509534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096e6e0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18901192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e83897dffe","size":5,"soft_fail":false,"target_va":5378598624,"va":5397499811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096f020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18905209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e88787dffe","size":5,"soft_fail":false,"target_va":5378600992,"va":5397506196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096f460
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18884926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8c2d6dffe","size":5,"soft_fail":false,"target_va":5378602080,"va":5397487001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409710b0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18882026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e816e2dffe","size":5,"soft_fail":false,"target_va":5378609328,"va":5397491349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409789a0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18851053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8135be0fe","size":5,"soft_fail":false,"target_va":5378640288,"va":5397491336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097ce50
//     owner=unresolved_namespace; count=6
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18845948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8046fe0fe","size":5,"soft_fail":false,"target_va":5378657872,"va":5397503815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097dd50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18843825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e84f77e0fe","size":5,"soft_fail":false,"target_va":5378661712,"va":5397505532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097e6e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18830015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e841ade0fe","size":5,"soft_fail":false,"target_va":5378664160,"va":5397494170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409961f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18729288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8b836e2fe","size":5,"soft_fail":false,"target_va":5378761200,"va":5397490483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099a4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18718274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8be61e2fe","size":5,"soft_fail":false,"target_va":5378778304,"va":5397496573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099a710
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18706786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e89e8ee2fe","size":5,"soft_fail":false,"target_va":5378778896,"va":5397485677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099e890
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18708120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e86889e2fe","size":5,"soft_fail":false,"target_va":5378795664,"va":5397503779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099e900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18700444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e864a7e2fe","size":5,"soft_fail":false,"target_va":5378795776,"va":5397496215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a3790
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18694540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e874bee2fe","size":5,"soft_fail":false,"target_va":5378815888,"va":5397510423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e6740
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18398781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8c341e7fe","size":5,"soft_fail":false,"target_va":5379090240,"va":5397489016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a0fa80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18241320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8d8a8e9fe","size":5,"soft_fail":false,"target_va":5379259008,"va":5397500323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a0fca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18238573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e893b3e9fe","size":5,"soft_fail":false,"target_va":5379259552,"va":5397498120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17774198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e88ac9f0fe","size":5,"soft_fail":false,"target_va":5379716592,"va":5397490785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b1d970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17123316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e80cb8fafe","size":5,"soft_fail":false,"target_va":5380364656,"va":5397487967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc7e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16426609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e88f5905ff","size":5,"soft_fail":false,"target_va":5381062208,"va":5397488812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db7490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14409681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e82f2024ff","size":5,"soft_fail":false,"target_va":5383091344,"va":5397501020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ae740
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10237956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8fcc763ff","size":5,"soft_fail":false,"target_va":5387249472,"va":5397487423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411af640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10232073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8f7de63ff","size":5,"soft_fail":false,"target_va":5387253312,"va":5397485380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b0bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10238522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8c6c563ff","size":5,"soft_fail":false,"target_va":5387258864,"va":5397497381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411c5b90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10158047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8210065ff","size":5,"soft_fail":false,"target_va":5387344784,"va":5397502826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141606b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5693768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8b81ea9ff","size":5,"soft_fail":false,"target_va":5391805296,"va":5397499059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4716971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e85506b8ff","size":5,"soft_fail":false,"target_va":5392780768,"va":5397497734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417afdf0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3951350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e80ab5c3ff","size":5,"soft_fail":false,"target_va":5393546736,"va":5397498081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417bb7c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3903683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e83d6fc4ff","size":5,"soft_fail":false,"target_va":5393594304,"va":5397497982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141824800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3471572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e82c07cbff","size":5,"soft_fail":false,"target_va":5394024448,"va":5397496015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141977c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2080413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e86341e0ff","size":5,"soft_fail":false,"target_va":5395414064,"va":5397494472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d65fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18830586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8fa541f01","size":5,"soft_fail":false,"target_va":5416312752,"va":5397482161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18850513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8d1a21f01","size":5,"soft_fail":false,"target_va":5416353040,"va":5397502522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=19
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18883601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e811242001","size":5,"soft_fail":false,"target_va":5416385024,"va":5397501418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7ecf0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18913222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8c6972001","size":5,"soft_fail":false,"target_va":5416414448,"va":5397501221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d905d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18985369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e899b12101","size":5,"soft_fail":false,"target_va":5416486352,"va":5397500978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd38d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19274680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8b81b2601","size":5,"soft_fail":false,"target_va":5416761552,"va":5397486867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd46a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19262092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e88cea2501","size":5,"soft_fail":false,"target_va":5416765088,"va":5397502991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd4bb0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19278194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e872292601","size":5,"soft_fail":false,"target_va":5416766384,"va":5397488185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd5460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19267761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8b1002601","size":5,"soft_fail":false,"target_va":5416768608,"va":5397500842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd56b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19271040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8800d2601","size":5,"soft_fail":false,"target_va":5416769200,"va":5397498155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19569218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8429a2a01","size":5,"soft_fail":false,"target_va":5417057040,"va":5397487817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19571414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8d6a22a01","size":5,"soft_fail":false,"target_va":5417057056,"va":5397485637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19566366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e81e8f2a01","size":5,"soft_fail":false,"target_va":5417057088,"va":5397490717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19119484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e87cbd2301","size":5,"soft_fail":false,"target_va":5416620769,"va":5397501280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-12690253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8b35c3eff","size":5,"soft_fail":false,"target_va":5384794864,"va":5397485112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18874446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e84e002001","size":5,"soft_fail":false,"target_va":5416368112,"va":5397493661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19117693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e87db62301","size":5,"soft_fail":false,"target_va":5416620794,"va":5397503096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-15370268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8e47715ff","size":5,"soft_fail":false,"target_va":5382122800,"va":5397493063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12862845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e883ba3bff","size":5,"soft_fail":false,"target_va":5384624080,"va":5397486920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=16
//     evidence={"assembly":"\tcallq\t19120514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e882c12301","size":5,"soft_fail":false,"target_va":5416621408,"va":5397500889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19120693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e835c22301","size":5,"soft_fail":false,"target_va":5416621600,"va":5397500902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19120886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8f6c22301","size":5,"soft_fail":false,"target_va":5416622208,"va":5397501317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19119379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e813bd2301","size":5,"soft_fail":false,"target_va":5416622240,"va":5397502856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<alloc::boxed::Box<country_core::systems::save::save_game::GameSaveData,alloc::alloc::Global> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-17083881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e81752fbfe","size":5,"soft_fail":false,"target_va":5380419760,"va":5397503636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<alloc::boxed::Box<country_core::systems::save::snapshot::SnapshotSaveData,alloc::alloc::Global> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17457104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e830a0f5fe","size":5,"soft_fail":false,"target_va":5380060976,"va":5397518075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> > > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8d2510000","size":5,"soft_fail":false,"target_va":5397525424,"va":5397504473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::ReadDir,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14592531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8ed5521ff","size":5,"soft_fail":false,"target_va":5382926368,"va":5397518894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19119415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e837bd2301","size":5,"soft_fail":false,"target_va":5416621632,"va":5397502212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15632808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8587611ff","size":5,"soft_fail":false,"target_va":5381870752,"va":5397503555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12663431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e879c53eff","size":5,"soft_fail":false,"target_va":5384830080,"va":5397493506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8b09d0000","size":5,"soft_fail":false,"target_va":5397526336,"va":5397485963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t19118234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e89ab82301","size":5,"soft_fail":false,"target_va":5416621376,"va":5397503137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::time::Duration::from_secs_f32
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-26347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e81599ffff","size":5,"soft_fail":false,"target_va":5397470432,"va":5397496774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::backup_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17217141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e88b49f9fe","size":5,"soft_fail":false,"target_va":5380267776,"va":5397484912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17217621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8ab47f9fe","size":5,"soft_fail":false,"target_va":5380267312,"va":5397484928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_game::add_history_edit_to_save_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17453622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8caadf5fe","size":5,"soft_fail":false,"target_va":5380036432,"va":5397490049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::CurrentSaveFile::change_section
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e823d6ffff","size":5,"soft_fail":false,"target_va":5397475792,"va":5397486504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8b3e5ffff","size":5,"soft_fail":false,"target_va":5397478496,"va":5397485224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::UnsavedFileList::add
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e89cdaffff","size":5,"soft_fail":false,"target_va":5397481664,"va":5397491231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::shadow_file::ShadowFile::create
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17773364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8ccccf0fe","size":5,"soft_fail":false,"target_va":5379721328,"va":5397494687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::shadow_file::ShadowFile::flush
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17769169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e82fddf0fe","size":5,"soft_fail":false,"target_va":5379717360,"va":5397486524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::snapshot::capture_game_snapshot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t531577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8791c0800","size":5,"soft_fail":false,"target_va":5398020320,"va":5397488738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19075432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e868112301","size":5,"soft_fail":false,"target_va":5416576832,"va":5397501395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19073454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8ae092301","size":5,"soft_fail":false,"target_va":5416576144,"va":5397502685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13502764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8d4f631ff","size":5,"soft_fail":false,"target_va":5383984848,"va":5397487607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13502868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e86cf631ff","size":5,"soft_fail":false,"target_va":5383985360,"va":5397488223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13521061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e85baf31ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5397484848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=19
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13512547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e89dd031ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5397484766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13521714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8ceac31ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5397486621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13519723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e895b431ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5397482534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-12735554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8beab3dff","size":5,"soft_fail":false,"target_va":5384752464,"va":5397488013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::copy
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12761567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e821463dff","size":5,"soft_fail":false,"target_va":5384723728,"va":5397485290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12767238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8fa2f3dff","size":5,"soft_fail":false,"target_va":5384725152,"va":5397492385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::remove_file
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12776744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e8d80a3dff","size":5,"soft_fail":false,"target_va":5384722736,"va":5397499475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12774247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e899143dff","size":5,"soft_fail":false,"target_va":5384726688,"va":5397500930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=16
//     evidence={"assembly":"\tcallq\t19117908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397482144,"raw_hex":"e854b72301","size":5,"soft_fail":false,"target_va":5416619360,"va":5397501447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5397482144,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5397486405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":1800,"record_offset":68388,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":74988,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":36280,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":44820,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":57020,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":65980,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":65908,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1800,"record_offset":73608,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":73692,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MaybePendingSnapshotId::assign
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1800,"record_offset":113412,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":35962,"source_line":185,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MaybePendingSnapshotId::assign_invalid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1800,"record_offset":110536,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":35961,"source_line":189,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1800,"record_offset":70356,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":33653,"source_line":122,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_game::SaveFileSection::save_cooldown_seconds
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_game.rs
//     evidence={"module":1800,"record_offset":65952,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":34526,"source_line":568,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::CurrentSaveFile::backup_save
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":46000,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":87664,"source_line":187,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::CurrentSaveFile::perform_manual_backup
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":70468,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":87666,"source_line":266,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::CurrentSaveFile::replace_edits
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":57976,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":87665,"source_line":65,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::capture_snapshot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":77864,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":87668,"source_line":796,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::execute_open_file_cmd
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":84156,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":87669,"source_line":670,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":74160,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":33689,"source_line":112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":73748,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":33690,"source_line":247,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":1800,"record_offset":73448,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":36336,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":36196,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":45168,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":36448,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":75296,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":74188,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":73052,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$149::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":73776,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":34011,"source_line":3856,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::SyncSender::send
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":1800,"record_offset":68416,"parent_record_offset":35936,"ancestor_record_offset":35936,"inlinee":13461,"source_line":673,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":97844,"parent_record_offset":97820,"ancestor_record_offset":35936,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":97868,"parent_record_offset":97844,"ancestor_record_offset":35936,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":46564,"parent_record_offset":46540,"ancestor_record_offset":35936,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":46588,"parent_record_offset":46564,"ancestor_record_offset":35936,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":97892,"parent_record_offset":97868,"ancestor_record_offset":35936,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":46612,"parent_record_offset":46588,"ancestor_record_offset":35936,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":97820,"parent_record_offset":97788,"ancestor_record_offset":35936,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":46540,"parent_record_offset":46484,"ancestor_record_offset":35936,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1800,"record_offset":97788,"parent_record_offset":97744,"ancestor_record_offset":35936,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1800,"record_offset":97744,"parent_record_offset":97708,"ancestor_record_offset":35936,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1800,"record_offset":67220,"parent_record_offset":67188,"ancestor_record_offset":35936,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1800,"record_offset":46684,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1800,"record_offset":46732,"parent_record_offset":46708,"ancestor_record_offset":35936,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":62688,"parent_record_offset":62664,"ancestor_record_offset":35936,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":51396,"parent_record_offset":51352,"ancestor_record_offset":35936,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":62832,"parent_record_offset":62804,"ancestor_record_offset":35936,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":98216,"parent_record_offset":98180,"ancestor_record_offset":35936,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":62712,"parent_record_offset":62688,"ancestor_record_offset":35936,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=73
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":46512,"parent_record_offset":46484,"ancestor_record_offset":35936,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=75
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":46484,"parent_record_offset":46460,"ancestor_record_offset":35936,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":62892,"parent_record_offset":62860,"ancestor_record_offset":35936,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":51452,"parent_record_offset":51424,"ancestor_record_offset":35936,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":51424,"parent_record_offset":51396,"ancestor_record_offset":35936,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":62860,"parent_record_offset":62832,"ancestor_record_offset":35936,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":98292,"parent_record_offset":98252,"ancestor_record_offset":35936,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":98252,"parent_record_offset":98216,"ancestor_record_offset":35936,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=75
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":46460,"parent_record_offset":46436,"ancestor_record_offset":35936,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":51132,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":98068,"parent_record_offset":98020,"ancestor_record_offset":35936,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":98104,"parent_record_offset":98068,"ancestor_record_offset":35936,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":98140,"parent_record_offset":98104,"ancestor_record_offset":35936,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":98020,"parent_record_offset":97984,"ancestor_record_offset":35936,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":51168,"parent_record_offset":51132,"ancestor_record_offset":35936,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1800,"record_offset":97984,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1800,"record_offset":89108,"parent_record_offset":89072,"ancestor_record_offset":35936,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1800,"record_offset":89072,"parent_record_offset":89048,"ancestor_record_offset":35936,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":63012,"parent_record_offset":62988,"ancestor_record_offset":35936,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":51352,"parent_record_offset":51324,"ancestor_record_offset":35936,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":62664,"parent_record_offset":62608,"ancestor_record_offset":35936,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":62776,"parent_record_offset":62608,"ancestor_record_offset":35936,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":87068,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":51324,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":91432,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":91464,"parent_record_offset":91432,"ancestor_record_offset":35936,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":62804,"parent_record_offset":62608,"ancestor_record_offset":35936,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":63192,"parent_record_offset":62608,"ancestor_record_offset":35936,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":62988,"parent_record_offset":62608,"ancestor_record_offset":35936,"inlinee":9137,"source_line":3195,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":60696,"parent_record_offset":57976,"ancestor_record_offset":35936,"inlinee":9755,"source_line":1761,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":98180,"parent_record_offset":98140,"ancestor_record_offset":35936,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":51096,"parent_record_offset":51068,"ancestor_record_offset":35936,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":60728,"parent_record_offset":57976,"ancestor_record_offset":35936,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":61544,"parent_record_offset":61504,"ancestor_record_offset":35936,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":61504,"parent_record_offset":61472,"ancestor_record_offset":35936,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":61800,"parent_record_offset":61776,"ancestor_record_offset":35936,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":60784,"parent_record_offset":57976,"ancestor_record_offset":35936,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":61616,"parent_record_offset":61592,"ancestor_record_offset":35936,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1800,"record_offset":96120,"parent_record_offset":96092,"ancestor_record_offset":35936,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1800,"record_offset":48928,"parent_record_offset":48884,"ancestor_record_offset":35936,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1800,"record_offset":93048,"parent_record_offset":93020,"ancestor_record_offset":35936,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":48884,"parent_record_offset":48848,"ancestor_record_offset":35936,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":93020,"parent_record_offset":92984,"ancestor_record_offset":35936,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":48848,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":1800,"record_offset":63592,"parent_record_offset":63568,"ancestor_record_offset":35936,"inlinee":27267,"source_line":333,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":1800,"record_offset":63648,"parent_record_offset":63624,"ancestor_record_offset":35936,"inlinee":27254,"source_line":350,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":1800,"record_offset":63568,"parent_record_offset":63540,"ancestor_record_offset":35936,"inlinee":27318,"source_line":774,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::set_len_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec.rs
//     evidence={"module":1800,"record_offset":63624,"parent_record_offset":63540,"ancestor_record_offset":35936,"inlinee":27253,"source_line":585,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":1800,"record_offset":63540,"parent_record_offset":63388,"ancestor_record_offset":35936,"inlinee":33636,"source_line":359,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1800,"record_offset":67256,"parent_record_offset":67220,"ancestor_record_offset":35936,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1800,"record_offset":70404,"parent_record_offset":70380,"ancestor_record_offset":35936,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1800,"record_offset":70428,"parent_record_offset":70404,"ancestor_record_offset":35936,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":45504,"parent_record_offset":45476,"ancestor_record_offset":35936,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":95448,"parent_record_offset":95420,"ancestor_record_offset":35936,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=59
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":45532,"parent_record_offset":45504,"ancestor_record_offset":35936,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":45452,"parent_record_offset":45424,"ancestor_record_offset":35936,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":45476,"parent_record_offset":45452,"ancestor_record_offset":35936,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":45592,"parent_record_offset":45476,"ancestor_record_offset":35936,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":45704,"parent_record_offset":45680,"ancestor_record_offset":35936,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":90616,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":48708,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":73976,"parent_record_offset":73824,"ancestor_record_offset":35936,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1800,"record_offset":47464,"parent_record_offset":47428,"ancestor_record_offset":35936,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1800,"record_offset":85800,"parent_record_offset":85776,"ancestor_record_offset":35936,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1800,"record_offset":46768,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1800,"record_offset":51248,"parent_record_offset":51204,"ancestor_record_offset":35936,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1800,"record_offset":60616,"parent_record_offset":60532,"ancestor_record_offset":35936,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\cloned.rs
//     evidence={"module":1800,"record_offset":95868,"parent_record_offset":95840,"ancestor_record_offset":35936,"inlinee":6736,"source_line":57,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1800,"record_offset":88792,"parent_record_offset":88724,"ancestor_record_offset":35936,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$11::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1800,"record_offset":88424,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":11834,"source_line":225,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1800,"record_offset":88724,"parent_record_offset":88680,"ancestor_record_offset":35936,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$46::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1800,"record_offset":88476,"parent_record_offset":88424,"ancestor_record_offset":35936,"inlinee":11836,"source_line":851,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1800,"record_offset":88588,"parent_record_offset":88536,"ancestor_record_offset":35936,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1800,"record_offset":88536,"parent_record_offset":88476,"ancestor_record_offset":35936,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1800,"record_offset":95840,"parent_record_offset":95812,"ancestor_record_offset":35936,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1800,"record_offset":95988,"parent_record_offset":95924,"ancestor_record_offset":35936,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1800,"record_offset":74088,"parent_record_offset":74060,"ancestor_record_offset":35936,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1800,"record_offset":61592,"parent_record_offset":61568,"ancestor_record_offset":35936,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1800,"record_offset":61568,"parent_record_offset":61544,"ancestor_record_offset":35936,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1800,"record_offset":51068,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1800,"record_offset":95812,"parent_record_offset":95680,"ancestor_record_offset":35936,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1800,"record_offset":96188,"parent_record_offset":96164,"ancestor_record_offset":35936,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1800,"record_offset":74060,"parent_record_offset":73824,"ancestor_record_offset":35936,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1800,"record_offset":67324,"parent_record_offset":67288,"ancestor_record_offset":35936,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1800,"record_offset":45556,"parent_record_offset":45532,"ancestor_record_offset":35936,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1800,"record_offset":67288,"parent_record_offset":67256,"ancestor_record_offset":35936,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1800,"record_offset":63440,"parent_record_offset":63388,"ancestor_record_offset":35936,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":84936,"parent_record_offset":84908,"ancestor_record_offset":35936,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":84964,"parent_record_offset":84936,"ancestor_record_offset":35936,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":62748,"parent_record_offset":62608,"ancestor_record_offset":35936,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":96256,"parent_record_offset":95924,"ancestor_record_offset":35936,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":61928,"parent_record_offset":61900,"ancestor_record_offset":35936,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":62916,"parent_record_offset":62892,"ancestor_record_offset":35936,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":60532,"parent_record_offset":60508,"ancestor_record_offset":35936,"inlinee":14986,"source_line":863,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":63508,"parent_record_offset":63388,"ancestor_record_offset":35936,"inlinee":14756,"source_line":3657,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":60584,"parent_record_offset":60560,"ancestor_record_offset":35936,"inlinee":14988,"source_line":2828,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":60560,"parent_record_offset":60532,"ancestor_record_offset":35936,"inlinee":14987,"source_line":2905,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":95476,"parent_record_offset":95420,"ancestor_record_offset":35936,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1800,"record_offset":86020,"parent_record_offset":85996,"ancestor_record_offset":35936,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1800,"record_offset":96012,"parent_record_offset":95988,"ancestor_record_offset":35936,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1800,"record_offset":45320,"parent_record_offset":45276,"ancestor_record_offset":35936,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":90472,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":88640,"parent_record_offset":88588,"ancestor_record_offset":35936,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":73916,"parent_record_offset":73892,"ancestor_record_offset":35936,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":60440,"parent_record_offset":57976,"ancestor_record_offset":35936,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":90240,"parent_record_offset":90136,"ancestor_record_offset":35936,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":90264,"parent_record_offset":90240,"ancestor_record_offset":35936,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":36304,"parent_record_offset":36280,"ancestor_record_offset":35936,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":46708,"parent_record_offset":46684,"ancestor_record_offset":35936,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":84468,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":51500,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":95152,"parent_record_offset":95120,"ancestor_record_offset":35936,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":90644,"parent_record_offset":90616,"ancestor_record_offset":35936,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":37412,"parent_record_offset":37384,"ancestor_record_offset":35936,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":45952,"parent_record_offset":45276,"ancestor_record_offset":35936,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":98452,"parent_record_offset":98140,"ancestor_record_offset":35936,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":61688,"parent_record_offset":61664,"ancestor_record_offset":35936,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":63280,"parent_record_offset":63252,"ancestor_record_offset":35936,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":98476,"parent_record_offset":98452,"ancestor_record_offset":35936,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=550
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":45656,"parent_record_offset":45424,"ancestor_record_offset":35936,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":60752,"parent_record_offset":60728,"ancestor_record_offset":35936,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":63252,"parent_record_offset":63224,"ancestor_record_offset":35936,"inlinee":9139,"source_line":1359,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":86288,"parent_record_offset":86264,"ancestor_record_offset":35936,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":61664,"parent_record_offset":61640,"ancestor_record_offset":35936,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":95244,"parent_record_offset":95120,"ancestor_record_offset":35936,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":85000,"parent_record_offset":84876,"ancestor_record_offset":35936,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":60852,"parent_record_offset":60784,"ancestor_record_offset":35936,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":89140,"parent_record_offset":89108,"ancestor_record_offset":35936,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":36872,"parent_record_offset":36848,"ancestor_record_offset":35936,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":61640,"parent_record_offset":61616,"ancestor_record_offset":35936,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":60880,"parent_record_offset":60784,"ancestor_record_offset":35936,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":36848,"parent_record_offset":36824,"ancestor_record_offset":35936,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":37056,"parent_record_offset":37028,"ancestor_record_offset":35936,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":60904,"parent_record_offset":60880,"ancestor_record_offset":35936,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":95272,"parent_record_offset":95244,"ancestor_record_offset":35936,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":91632,"parent_record_offset":91464,"ancestor_record_offset":35936,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":99800,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::inspect_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":88000,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":11771,"source_line":1016,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":50684,"parent_record_offset":50648,"ancestor_record_offset":35936,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":88356,"parent_record_offset":88328,"ancestor_record_offset":35936,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":60652,"parent_record_offset":60472,"ancestor_record_offset":35936,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":90044,"parent_record_offset":89992,"ancestor_record_offset":35936,"inlinee":33993,"source_line":2079,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":77980,"parent_record_offset":77864,"ancestor_record_offset":35936,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":78760,"parent_record_offset":77864,"ancestor_record_offset":35936,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$7::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":88328,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":33954,"source_line":1939,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":74000,"parent_record_offset":73976,"ancestor_record_offset":35936,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":37156,"parent_record_offset":37128,"ancestor_record_offset":35936,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":74024,"parent_record_offset":74000,"ancestor_record_offset":35936,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1800,"record_offset":37128,"parent_record_offset":37100,"ancestor_record_offset":35936,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1800,"record_offset":63136,"parent_record_offset":62608,"ancestor_record_offset":35936,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1800,"record_offset":92872,"parent_record_offset":92672,"ancestor_record_offset":35936,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1800,"record_offset":73632,"parent_record_offset":73608,"ancestor_record_offset":35936,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":36800,"parent_record_offset":36776,"ancestor_record_offset":35936,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":36824,"parent_record_offset":36800,"ancestor_record_offset":35936,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":37184,"parent_record_offset":37156,"ancestor_record_offset":35936,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":95924,"parent_record_offset":95896,"ancestor_record_offset":35936,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":36776,"parent_record_offset":36660,"ancestor_record_offset":35936,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1800,"record_offset":51204,"parent_record_offset":51168,"ancestor_record_offset":35936,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":1800,"record_offset":63160,"parent_record_offset":63136,"ancestor_record_offset":35936,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1800,"record_offset":37332,"parent_record_offset":37304,"ancestor_record_offset":35936,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1800,"record_offset":36484,"parent_record_offset":36448,"ancestor_record_offset":35936,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1800,"record_offset":36628,"parent_record_offset":36596,"ancestor_record_offset":35936,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1800,"record_offset":36596,"parent_record_offset":36556,"ancestor_record_offset":35936,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":36532,"parent_record_offset":36508,"ancestor_record_offset":35936,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":36556,"parent_record_offset":36532,"ancestor_record_offset":35936,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":36508,"parent_record_offset":36484,"ancestor_record_offset":35936,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":37100,"parent_record_offset":36556,"ancestor_record_offset":35936,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":58284,"parent_record_offset":58252,"ancestor_record_offset":35936,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":58252,"parent_record_offset":58220,"ancestor_record_offset":35936,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":37304,"parent_record_offset":37272,"ancestor_record_offset":35936,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":37360,"parent_record_offset":37304,"ancestor_record_offset":35936,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":37272,"parent_record_offset":37244,"ancestor_record_offset":35936,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":90696,"parent_record_offset":90672,"ancestor_record_offset":35936,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":37244,"parent_record_offset":36448,"ancestor_record_offset":35936,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":37384,"parent_record_offset":37360,"ancestor_record_offset":35936,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":36660,"parent_record_offset":36628,"ancestor_record_offset":35936,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":36944,"parent_record_offset":36660,"ancestor_record_offset":35936,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":36916,"parent_record_offset":36660,"ancestor_record_offset":35936,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":36720,"parent_record_offset":36660,"ancestor_record_offset":35936,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":110564,"parent_record_offset":110536,"ancestor_record_offset":35936,"inlinee":21426,"source_line":2870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":75228,"parent_record_offset":75164,"ancestor_record_offset":35936,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":73516,"parent_record_offset":73476,"ancestor_record_offset":35936,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":98936,"parent_record_offset":98900,"ancestor_record_offset":35936,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":36220,"parent_record_offset":36196,"ancestor_record_offset":35936,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":75252,"parent_record_offset":75228,"ancestor_record_offset":35936,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":73540,"parent_record_offset":73516,"ancestor_record_offset":35936,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":36244,"parent_record_offset":36220,"ancestor_record_offset":35936,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":110592,"parent_record_offset":110564,"ancestor_record_offset":35936,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":89168,"parent_record_offset":89108,"ancestor_record_offset":35936,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":89200,"parent_record_offset":89072,"ancestor_record_offset":35936,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1800,"record_offset":90572,"parent_record_offset":90548,"ancestor_record_offset":35936,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::HistorySnapshotId::file_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1800,"record_offset":78068,"parent_record_offset":77864,"ancestor_record_offset":35936,"inlinee":35957,"source_line":285,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::HistorySnapshotId::from_snapshot_blob
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1800,"record_offset":78012,"parent_record_offset":77864,"ancestor_record_offset":35936,"inlinee":35955,"source_line":281,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_backup_list::SaveBackupList::into_files
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":1800,"record_offset":94796,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":36928,"source_line":136,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":1800,"record_offset":50604,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":36869,"source_line":54,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::execute_open_file_cmd::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":88044,"parent_record_offset":88000,"ancestor_record_offset":35936,"inlinee":87670,"source_line":692,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::full_write_history_file
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":92672,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":87671,"source_line":750,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFile::data
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":48764,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":33689,"source_line":112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFile::truncate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":63388,"parent_record_offset":57976,"ancestor_record_offset":35936,"inlinee":33694,"source_line":103,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFile::write_at
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":60472,"parent_record_offset":57976,"ancestor_record_offset":35936,"inlinee":33695,"source_line":93,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":62492,"parent_record_offset":62452,"ancestor_record_offset":35936,"inlinee":33645,"source_line":289,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFileWriter::write_all
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":62452,"parent_record_offset":57976,"ancestor_record_offset":35936,"inlinee":33697,"source_line":281,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":85964,"parent_record_offset":85936,"ancestor_record_offset":35936,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":85996,"parent_record_offset":85964,"ancestor_record_offset":35936,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":85936,"parent_record_offset":85896,"ancestor_record_offset":35936,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":85776,"parent_record_offset":85748,"ancestor_record_offset":35936,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":95716,"parent_record_offset":95680,"ancestor_record_offset":35936,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":95680,"parent_record_offset":95648,"ancestor_record_offset":35936,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":96212,"parent_record_offset":96188,"ancestor_record_offset":35936,"inlinee":12038,"source_line":4661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":86264,"parent_record_offset":85660,"ancestor_record_offset":35936,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":86116,"parent_record_offset":85896,"ancestor_record_offset":35936,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":85748,"parent_record_offset":85692,"ancestor_record_offset":35936,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":85896,"parent_record_offset":85868,"ancestor_record_offset":35936,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":95744,"parent_record_offset":95716,"ancestor_record_offset":35936,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":84876,"parent_record_offset":84844,"ancestor_record_offset":35936,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":85720,"parent_record_offset":85692,"ancestor_record_offset":35936,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":85660,"parent_record_offset":85600,"ancestor_record_offset":35936,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":84788,"parent_record_offset":84764,"ancestor_record_offset":35936,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":84844,"parent_record_offset":84788,"ancestor_record_offset":35936,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":84816,"parent_record_offset":84788,"ancestor_record_offset":35936,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":85692,"parent_record_offset":85660,"ancestor_record_offset":35936,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":84908,"parent_record_offset":84876,"ancestor_record_offset":35936,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":84764,"parent_record_offset":84740,"ancestor_record_offset":35936,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":85868,"parent_record_offset":85660,"ancestor_record_offset":35936,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1800,"record_offset":50176,"parent_record_offset":50152,"ancestor_record_offset":35936,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1800,"record_offset":95648,"parent_record_offset":95616,"ancestor_record_offset":35936,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":1800,"record_offset":75136,"parent_record_offset":75108,"ancestor_record_offset":35936,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":36384,"parent_record_offset":36360,"ancestor_record_offset":35936,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":51696,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":36360,"parent_record_offset":36336,"ancestor_record_offset":35936,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":36408,"parent_record_offset":36384,"ancestor_record_offset":35936,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":48624,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1800,"record_offset":73476,"parent_record_offset":73448,"ancestor_record_offset":35936,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1800,"record_offset":75164,"parent_record_offset":75136,"ancestor_record_offset":35936,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1800,"record_offset":73572,"parent_record_offset":73476,"ancestor_record_offset":35936,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1800,"record_offset":75200,"parent_record_offset":75164,"ancestor_record_offset":35936,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":59588,"parent_record_offset":57976,"ancestor_record_offset":35936,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":45204,"parent_record_offset":45168,"ancestor_record_offset":35936,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":58176,"parent_record_offset":57976,"ancestor_record_offset":35936,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":64036,"parent_record_offset":64004,"ancestor_record_offset":35936,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":64516,"parent_record_offset":64300,"ancestor_record_offset":35936,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=19
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1800,"record_offset":45348,"parent_record_offset":45320,"ancestor_record_offset":35936,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":45424,"parent_record_offset":45276,"ancestor_record_offset":35936,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":45628,"parent_record_offset":45424,"ancestor_record_offset":35936,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":58572,"parent_record_offset":57976,"ancestor_record_offset":35936,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":50152,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":95588,"parent_record_offset":94860,"ancestor_record_offset":35936,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":95616,"parent_record_offset":94860,"ancestor_record_offset":35936,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":94860,"parent_record_offset":94824,"ancestor_record_offset":35936,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1800,"record_offset":90524,"parent_record_offset":90500,"ancestor_record_offset":35936,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1800,"record_offset":96064,"parent_record_offset":96036,"ancestor_record_offset":35936,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":78572,"parent_record_offset":78492,"ancestor_record_offset":35936,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":78516,"parent_record_offset":78492,"ancestor_record_offset":35936,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":47428,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":78492,"parent_record_offset":77864,"ancestor_record_offset":35936,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":50648,"parent_record_offset":50604,"ancestor_record_offset":35936,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":51820,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":48792,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1800,"record_offset":94940,"parent_record_offset":94912,"ancestor_record_offset":35936,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1800,"record_offset":95204,"parent_record_offset":95180,"ancestor_record_offset":35936,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1800,"record_offset":95060,"parent_record_offset":95032,"ancestor_record_offset":35936,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1800,"record_offset":95420,"parent_record_offset":94996,"ancestor_record_offset":35936,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1800,"record_offset":94912,"parent_record_offset":94860,"ancestor_record_offset":35936,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::into_inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":93096,"parent_record_offset":92672,"ancestor_record_offset":35936,"inlinee":87672,"source_line":116,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":92788,"parent_record_offset":92672,"ancestor_record_offset":35936,"inlinee":23648,"source_line":98,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":60508,"parent_record_offset":60472,"ancestor_record_offset":35936,"inlinee":15004,"source_line":293,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$9::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":62524,"parent_record_offset":62452,"ancestor_record_offset":35936,"inlinee":29134,"source_line":674,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":62608,"parent_record_offset":62556,"ancestor_record_offset":35936,"inlinee":9136,"source_line":464,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":62556,"parent_record_offset":62524,"ancestor_record_offset":35936,"inlinee":9135,"source_line":528,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":63224,"parent_record_offset":62556,"ancestor_record_offset":35936,"inlinee":9138,"source_line":510,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":1800,"record_offset":80352,"parent_record_offset":80328,"ancestor_record_offset":35936,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":73892,"parent_record_offset":73824,"ancestor_record_offset":35936,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":46288,"parent_record_offset":46000,"ancestor_record_offset":35936,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":73864,"parent_record_offset":73824,"ancestor_record_offset":35936,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":73824,"parent_record_offset":73800,"ancestor_record_offset":35936,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":96036,"parent_record_offset":96012,"ancestor_record_offset":35936,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$76::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":73800,"parent_record_offset":73776,"ancestor_record_offset":35936,"inlinee":34013,"source_line":3652,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":1800,"record_offset":97672,"parent_record_offset":97636,"ancestor_record_offset":35936,"inlinee":5896,"source_line":198,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\counter.rs
//     evidence={"module":1800,"record_offset":97708,"parent_record_offset":97672,"ancestor_record_offset":35936,"inlinee":5885,"source_line":20,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpsc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":1800,"record_offset":97636,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":7548,"source_line":519,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1800,"record_offset":90548,"parent_record_offset":90524,"ancestor_record_offset":35936,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1800,"record_offset":96092,"parent_record_offset":96064,"ancestor_record_offset":35936,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1800,"record_offset":45380,"parent_record_offset":45348,"ancestor_record_offset":35936,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1800,"record_offset":95120,"parent_record_offset":95088,"ancestor_record_offset":35936,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":1800,"record_offset":97956,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":7549,"source_line":78,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":1800,"record_offset":98528,"parent_record_offset":84156,"ancestor_record_offset":35936,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":1800,"record_offset":98556,"parent_record_offset":98528,"ancestor_record_offset":35936,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1800,"record_offset":45276,"parent_record_offset":45240,"ancestor_record_offset":35936,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1800,"record_offset":45240,"parent_record_offset":45204,"ancestor_record_offset":35936,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":1800,"record_offset":70380,"parent_record_offset":70356,"ancestor_record_offset":35936,"inlinee":11456,"source_line":437,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::save_thread
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1800,"record_offset":1672,"rva":28761936,"va":5397471056}
// 编译实例 RVAs: 0x1b6df50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12826223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e891493cff","size":5,"soft_fail":false,"target_va":5384644912,"va":5397471130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12817159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8f96c3cff","size":5,"soft_fail":false,"target_va":5384656240,"va":5397473394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12817778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e88e6a3cff","size":5,"soft_fail":false,"target_va":5384653728,"va":5397471501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant as core::ops::arith::Add<core::time::Duration>>::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12702909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8432b3eff","size":5,"soft_fail":false,"target_va":5384768864,"va":5397471768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12823180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e874553cff","size":5,"soft_fail":false,"target_va":5384648032,"va":5397471207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12862649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e847bb3bff","size":5,"soft_fail":false,"target_va":5384609408,"va":5397472052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=26
//     evidence={"assembly":"\tcallq\t-21793685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e86b74b3fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397472384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28169384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8582b52fe","size":5,"soft_fail":false,"target_va":5369305584,"va":5397474963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bc230
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27995016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e878d454fe","size":5,"soft_fail":false,"target_va":5369479728,"va":5397474739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14091bbf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19212326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8dad7dafe","size":5,"soft_fail":false,"target_va":5378259952,"va":5397472273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096edf0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18873977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e88701e0fe","size":5,"soft_fail":false,"target_va":5378600432,"va":5397474404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140971f80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18861276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e82433e0fe","size":5,"soft_fail":false,"target_va":5378613120,"va":5397474391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097ce50
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18816808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8d8e0e0fe","size":5,"soft_fail":false,"target_va":5378657872,"va":5397474675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099e900
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18678227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e82dfee2fe","size":5,"soft_fail":false,"target_va":5378795776,"va":5397473998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfbb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16198471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8b9d408ff","size":5,"soft_fail":false,"target_va":5381274432,"va":5397472898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfbdf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16198060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e854d608ff","size":5,"soft_fail":false,"target_va":5381275120,"va":5397473175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c3b2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15938550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e80acc0cff","size":5,"soft_fail":false,"target_va":5381534368,"va":5397472913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c3b780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15937548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8f4cf0cff","size":5,"soft_fail":false,"target_va":5381535616,"va":5397473159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90900
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15590223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8b11c12ff","size":5,"soft_fail":false,"target_va":5381884160,"va":5397474378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9e510
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15534408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8b8f612ff","size":5,"soft_fail":false,"target_va":5381940496,"va":5397474899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca4830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15507137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e83f6113ff","size":5,"soft_fail":false,"target_va":5381965872,"va":5397473004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f1850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5753272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e84836a8ff","size":5,"soft_fail":false,"target_va":5391718480,"va":5397471747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f7c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5728127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e88198a8ff","size":5,"soft_fail":false,"target_va":5391744128,"va":5397472250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b1840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3919638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8ea30c4ff","size":5,"soft_fail":false,"target_va":5393553472,"va":5397473105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b3400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3912407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8294dc4ff","size":5,"soft_fail":false,"target_va":5393560576,"va":5397472978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417bd440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3870217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8f7f1c4ff","size":5,"soft_fail":false,"target_va":5393601600,"va":5397471812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b6de50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e82dfdffff","size":5,"soft_fail":false,"target_va":5397470800,"va":5397471518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18879894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e896152001","size":5,"soft_fail":false,"target_va":5416351504,"va":5397471605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19583269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e825d12a01","size":5,"soft_fail":false,"target_va":5417057056,"va":5397473782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19147638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8762b2401","size":5,"soft_fail":false,"target_va":5416622208,"va":5397474565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19147697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8b12b2401","size":5,"soft_fail":false,"target_va":5416622240,"va":5397474538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::save::save_game::AsyncSaveCmd> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t50175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8ffc30000","size":5,"soft_fail":false,"target_va":5397525472,"va":5397475292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17206037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8eb74f9fe","size":5,"soft_fail":false,"target_va":5380267312,"va":5397473344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::CurrentSaveFile::change_section
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e85e080000","size":5,"soft_fail":false,"target_va":5397475792,"va":5397473645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e858120000","size":5,"soft_fail":false,"target_va":5397478496,"va":5397473795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::UnsavedFileList::contains_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8471e0000","size":5,"soft_fail":false,"target_va":5397481184,"va":5397473428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::execute_save_cmd
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8222a0000","size":5,"soft_fail":false,"target_va":5397482144,"va":5397471353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::shadow_file::ShadowFile::sync_all
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17746747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8c534f1fe","size":5,"soft_fail":false,"target_va":5379724720,"va":5397471462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19103125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8957d2301","size":5,"soft_fail":false,"target_va":5416576832,"va":5397473702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19103512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8187f2301","size":5,"soft_fail":false,"target_va":5416576144,"va":5397472627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13486877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e8e33432ff","size":5,"soft_fail":false,"target_va":5383984848,"va":5397471720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13487233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e87f3332ff","size":5,"soft_fail":false,"target_va":5383985360,"va":5397472588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2788968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e89871d5ff","size":5,"soft_fail":false,"target_va":5394683920,"va":5397472883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12747747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e81d7c3dff","size":5,"soft_fail":false,"target_va":5384726688,"va":5397474430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12745065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397471056,"raw_hex":"e897863dff","size":5,"soft_fail":false,"target_va":5384728160,"va":5397473220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1800,"record_offset":2660,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":2972,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":3440,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":2688,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":15279,"source_line":1446,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":3712,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1800,"record_offset":2944,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":2860,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":9888,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":9916,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":10120,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":4132,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":4104,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::AutosaveErrorState::is_ok
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":3568,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":87657,"source_line":328,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::impl$10::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":4048,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":87659,"source_line":322,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::impl$11::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":2632,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":87653,"source_line":322,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":2888,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":33690,"source_line":247,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":1800,"record_offset":9832,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":10549,"source_line":221,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":1800,"record_offset":3228,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":1800,"record_offset":3384,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":7519,"source_line":696,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":3768,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":3628,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":1920,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":5639,"source_line":149,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":8032,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":13816,"source_line":1042,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":8916,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":27433,"source_line":479,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":9068,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":2916,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":1800,"record_offset":7968,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":7498,"source_line":789,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":1800,"record_offset":4076,"parent_record_offset":1672,"ancestor_record_offset":1672,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":4548,"parent_record_offset":4524,"ancestor_record_offset":1672,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":1672,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":4596,"parent_record_offset":4572,"ancestor_record_offset":1672,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":4524,"parent_record_offset":4460,"ancestor_record_offset":1672,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":3076,"parent_record_offset":3044,"ancestor_record_offset":1672,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":4492,"parent_record_offset":4460,"ancestor_record_offset":1672,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":4460,"parent_record_offset":4432,"ancestor_record_offset":1672,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":1672,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":1672,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":4432,"parent_record_offset":4404,"ancestor_record_offset":1672,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1800,"record_offset":13600,"parent_record_offset":13564,"ancestor_record_offset":1672,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1800,"record_offset":13564,"parent_record_offset":13540,"ancestor_record_offset":1672,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":3044,"parent_record_offset":3000,"ancestor_record_offset":1672,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":3596,"parent_record_offset":3568,"ancestor_record_offset":1672,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":3000,"parent_record_offset":2972,"ancestor_record_offset":1672,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":4188,"parent_record_offset":4160,"ancestor_record_offset":1672,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1800,"record_offset":3496,"parent_record_offset":3468,"ancestor_record_offset":1672,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":1672,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":2388,"parent_record_offset":2184,"ancestor_record_offset":1672,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":1672,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":1672,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1800,"record_offset":12248,"parent_record_offset":12224,"ancestor_record_offset":1672,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1800,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":1672,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":13260,"parent_record_offset":13232,"ancestor_record_offset":1672,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":13288,"parent_record_offset":13260,"ancestor_record_offset":1672,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":5096,"parent_record_offset":5068,"ancestor_record_offset":1672,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":2512,"parent_record_offset":2460,"ancestor_record_offset":1672,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1800,"record_offset":12468,"parent_record_offset":12444,"ancestor_record_offset":1672,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1800,"record_offset":2096,"parent_record_offset":2060,"ancestor_record_offset":1672,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":1672,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":2216,"parent_record_offset":2184,"ancestor_record_offset":1672,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":9856,"parent_record_offset":9832,"ancestor_record_offset":1672,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":12280,"parent_record_offset":12196,"ancestor_record_offset":1672,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=125
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":4160,"parent_record_offset":4132,"ancestor_record_offset":1672,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":12736,"parent_record_offset":12712,"ancestor_record_offset":1672,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":2304,"parent_record_offset":2184,"ancestor_record_offset":1672,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":12588,"parent_record_offset":12564,"ancestor_record_offset":1672,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":13632,"parent_record_offset":13600,"ancestor_record_offset":1672,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":2328,"parent_record_offset":2304,"ancestor_record_offset":1672,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":3164,"parent_record_offset":3000,"ancestor_record_offset":1672,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":1672,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":6564,"parent_record_offset":6500,"ancestor_record_offset":1672,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":3292,"parent_record_offset":3256,"ancestor_record_offset":1672,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":3652,"parent_record_offset":3628,"ancestor_record_offset":1672,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":6588,"parent_record_offset":6564,"ancestor_record_offset":1672,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":3316,"parent_record_offset":3292,"ancestor_record_offset":1672,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":1672,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":13660,"parent_record_offset":13600,"ancestor_record_offset":1672,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":13692,"parent_record_offset":13564,"ancestor_record_offset":1672,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":1800,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":1672,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":3468,"parent_record_offset":3440,"ancestor_record_offset":1672,"inlinee":87655,"source_line":322,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::save_thread::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":9944,"parent_record_offset":9916,"ancestor_record_offset":1672,"inlinee":87660,"source_line":499,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":12412,"parent_record_offset":12384,"ancestor_record_offset":1672,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":12444,"parent_record_offset":12412,"ancestor_record_offset":1672,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":12384,"parent_record_offset":12344,"ancestor_record_offset":1672,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":12224,"parent_record_offset":12196,"ancestor_record_offset":1672,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":1672,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":8108,"parent_record_offset":8080,"ancestor_record_offset":1672,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":8136,"parent_record_offset":8108,"ancestor_record_offset":1672,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":12712,"parent_record_offset":12108,"ancestor_record_offset":1672,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":12564,"parent_record_offset":12344,"ancestor_record_offset":1672,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":12196,"parent_record_offset":12140,"ancestor_record_offset":1672,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":12344,"parent_record_offset":12316,"ancestor_record_offset":1672,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":13200,"parent_record_offset":13168,"ancestor_record_offset":1672,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":12168,"parent_record_offset":12140,"ancestor_record_offset":1672,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":12108,"parent_record_offset":12048,"ancestor_record_offset":1672,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":12048,"parent_record_offset":12024,"ancestor_record_offset":1672,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":13168,"parent_record_offset":12048,"ancestor_record_offset":1672,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":12080,"parent_record_offset":12048,"ancestor_record_offset":1672,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":12140,"parent_record_offset":12108,"ancestor_record_offset":1672,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":13232,"parent_record_offset":13200,"ancestor_record_offset":1672,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":12024,"parent_record_offset":12000,"ancestor_record_offset":1672,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":12316,"parent_record_offset":12108,"ancestor_record_offset":1672,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1800,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":1672,"inlinee":13817,"source_line":1175,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::retain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1800,"record_offset":8940,"parent_record_offset":8916,"ancestor_record_offset":1672,"inlinee":27434,"source_line":368,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":1800,"record_offset":6476,"parent_record_offset":6452,"ancestor_record_offset":1672,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":1672,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":1672,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":3840,"parent_record_offset":3816,"ancestor_record_offset":1672,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1800,"record_offset":3256,"parent_record_offset":3228,"ancestor_record_offset":1672,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1800,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":1672,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1800,"record_offset":3348,"parent_record_offset":3256,"ancestor_record_offset":1672,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1800,"record_offset":6536,"parent_record_offset":6500,"ancestor_record_offset":1672,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":1948,"parent_record_offset":1920,"ancestor_record_offset":1672,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1800,"record_offset":2004,"parent_record_offset":1976,"ancestor_record_offset":1672,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1800,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":1672,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1800,"record_offset":2124,"parent_record_offset":2096,"ancestor_record_offset":1672,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1800,"record_offset":2460,"parent_record_offset":2060,"ancestor_record_offset":1672,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1800,"record_offset":1976,"parent_record_offset":1948,"ancestor_record_offset":1672,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":1800,"record_offset":7992,"parent_record_offset":7968,"ancestor_record_offset":1672,"inlinee":5895,"source_line":916,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1800,"record_offset":2152,"parent_record_offset":2124,"ancestor_record_offset":1672,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1800,"record_offset":2184,"parent_record_offset":2152,"ancestor_record_offset":1672,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$20::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":1800,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":1672,"inlinee":6431,"source_line":13,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1800,"record_offset":2060,"parent_record_offset":2032,"ancestor_record_offset":1672,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1800,"record_offset":2032,"parent_record_offset":2004,"ancestor_record_offset":1672,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$24::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":1800,"record_offset":2744,"parent_record_offset":2688,"ancestor_record_offset":1672,"inlinee":6429,"source_line":154,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::CurrentSaveFile::replace_edits
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15671
// 首条位置: {"module":1800,"inlinee":87665}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":60696,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":9755,"source_line":1761,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":60728,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":60784,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":61024,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":60440,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":61192,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::truncate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":63388,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":33694,"source_line":103,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::write_at
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":60472,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":33695,"source_line":93,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":63360,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":33645,"source_line":289,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFileWriter::write_all
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":62452,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":33697,"source_line":281,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":61080,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":60940,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":59588,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":58176,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":58572,"parent_record_offset":57976,"ancestor_record_offset":57976,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":61984,"parent_record_offset":61960,"ancestor_record_offset":57976,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":62008,"parent_record_offset":61984,"ancestor_record_offset":57976,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":62032,"parent_record_offset":62008,"ancestor_record_offset":57976,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":61960,"parent_record_offset":61872,"ancestor_record_offset":57976,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":62688,"parent_record_offset":62664,"ancestor_record_offset":57976,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":63044,"parent_record_offset":63012,"ancestor_record_offset":57976,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":62832,"parent_record_offset":62804,"ancestor_record_offset":57976,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":62712,"parent_record_offset":62688,"ancestor_record_offset":57976,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":61900,"parent_record_offset":61872,"ancestor_record_offset":57976,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":61872,"parent_record_offset":61848,"ancestor_record_offset":57976,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":62892,"parent_record_offset":62860,"ancestor_record_offset":57976,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":63092,"parent_record_offset":63068,"ancestor_record_offset":57976,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":63068,"parent_record_offset":63044,"ancestor_record_offset":57976,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":62860,"parent_record_offset":62832,"ancestor_record_offset":57976,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":61848,"parent_record_offset":61824,"ancestor_record_offset":57976,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":63012,"parent_record_offset":62988,"ancestor_record_offset":57976,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":62664,"parent_record_offset":62608,"ancestor_record_offset":57976,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":62776,"parent_record_offset":62608,"ancestor_record_offset":57976,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":62804,"parent_record_offset":62608,"ancestor_record_offset":57976,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":63192,"parent_record_offset":62608,"ancestor_record_offset":57976,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":62988,"parent_record_offset":62608,"ancestor_record_offset":57976,"inlinee":9137,"source_line":3195,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":61544,"parent_record_offset":61504,"ancestor_record_offset":57976,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":61504,"parent_record_offset":61472,"ancestor_record_offset":57976,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":61800,"parent_record_offset":61776,"ancestor_record_offset":57976,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":61616,"parent_record_offset":61592,"ancestor_record_offset":57976,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":1800,"record_offset":63592,"parent_record_offset":63568,"ancestor_record_offset":57976,"inlinee":27267,"source_line":333,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":1800,"record_offset":63648,"parent_record_offset":63624,"ancestor_record_offset":57976,"inlinee":27254,"source_line":350,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":1800,"record_offset":63568,"parent_record_offset":63540,"ancestor_record_offset":57976,"inlinee":27318,"source_line":774,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::set_len_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec.rs
//     evidence={"module":1800,"record_offset":63624,"parent_record_offset":63540,"ancestor_record_offset":57976,"inlinee":27253,"source_line":585,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":1800,"record_offset":63540,"parent_record_offset":63388,"ancestor_record_offset":57976,"inlinee":33636,"source_line":359,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":59944,"parent_record_offset":59916,"ancestor_record_offset":57976,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":59972,"parent_record_offset":59944,"ancestor_record_offset":57976,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":59884,"parent_record_offset":59852,"ancestor_record_offset":57976,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":59916,"parent_record_offset":59884,"ancestor_record_offset":57976,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":60032,"parent_record_offset":59916,"ancestor_record_offset":57976,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":60144,"parent_record_offset":60120,"ancestor_record_offset":57976,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1800,"record_offset":60616,"parent_record_offset":60532,"ancestor_record_offset":57976,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1800,"record_offset":61592,"parent_record_offset":61568,"ancestor_record_offset":57976,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1800,"record_offset":61568,"parent_record_offset":61544,"ancestor_record_offset":57976,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1800,"record_offset":59996,"parent_record_offset":59972,"ancestor_record_offset":57976,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1800,"record_offset":63440,"parent_record_offset":63388,"ancestor_record_offset":57976,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":62748,"parent_record_offset":62608,"ancestor_record_offset":57976,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":61928,"parent_record_offset":61900,"ancestor_record_offset":57976,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":62916,"parent_record_offset":62892,"ancestor_record_offset":57976,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":60532,"parent_record_offset":60508,"ancestor_record_offset":57976,"inlinee":14986,"source_line":863,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":63508,"parent_record_offset":63388,"ancestor_record_offset":57976,"inlinee":14756,"source_line":3657,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":60584,"parent_record_offset":60560,"ancestor_record_offset":57976,"inlinee":14988,"source_line":2828,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":60560,"parent_record_offset":60532,"ancestor_record_offset":57976,"inlinee":14987,"source_line":2905,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1800,"record_offset":59752,"parent_record_offset":59704,"ancestor_record_offset":57976,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":61048,"parent_record_offset":61024,"ancestor_record_offset":57976,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":58520,"parent_record_offset":58492,"ancestor_record_offset":57976,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":60392,"parent_record_offset":59704,"ancestor_record_offset":57976,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":61688,"parent_record_offset":61664,"ancestor_record_offset":57976,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":63280,"parent_record_offset":63252,"ancestor_record_offset":57976,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":60096,"parent_record_offset":59852,"ancestor_record_offset":57976,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":60752,"parent_record_offset":60728,"ancestor_record_offset":57976,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":63252,"parent_record_offset":63224,"ancestor_record_offset":57976,"inlinee":9139,"source_line":1359,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":61664,"parent_record_offset":61640,"ancestor_record_offset":57976,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":60852,"parent_record_offset":60784,"ancestor_record_offset":57976,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":58996,"parent_record_offset":58972,"ancestor_record_offset":57976,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":61640,"parent_record_offset":61616,"ancestor_record_offset":57976,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":60880,"parent_record_offset":60784,"ancestor_record_offset":57976,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":58972,"parent_record_offset":58948,"ancestor_record_offset":57976,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":59180,"parent_record_offset":59152,"ancestor_record_offset":57976,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":60904,"parent_record_offset":60880,"ancestor_record_offset":57976,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":60652,"parent_record_offset":60472,"ancestor_record_offset":57976,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":59280,"parent_record_offset":59252,"ancestor_record_offset":57976,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1800,"record_offset":59252,"parent_record_offset":59224,"ancestor_record_offset":57976,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1800,"record_offset":63136,"parent_record_offset":62608,"ancestor_record_offset":57976,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":58924,"parent_record_offset":58900,"ancestor_record_offset":57976,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":58948,"parent_record_offset":58924,"ancestor_record_offset":57976,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":59308,"parent_record_offset":59280,"ancestor_record_offset":57976,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":58900,"parent_record_offset":58784,"ancestor_record_offset":57976,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":1800,"record_offset":63160,"parent_record_offset":63136,"ancestor_record_offset":57976,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1800,"record_offset":58308,"parent_record_offset":58284,"ancestor_record_offset":57976,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1800,"record_offset":58608,"parent_record_offset":58572,"ancestor_record_offset":57976,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1800,"record_offset":58752,"parent_record_offset":58720,"ancestor_record_offset":57976,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1800,"record_offset":58720,"parent_record_offset":58680,"ancestor_record_offset":57976,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":58656,"parent_record_offset":58632,"ancestor_record_offset":57976,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":58680,"parent_record_offset":58656,"ancestor_record_offset":57976,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":58632,"parent_record_offset":58608,"ancestor_record_offset":57976,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":59224,"parent_record_offset":58680,"ancestor_record_offset":57976,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":58284,"parent_record_offset":58252,"ancestor_record_offset":57976,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":58252,"parent_record_offset":58220,"ancestor_record_offset":57976,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":58412,"parent_record_offset":58380,"ancestor_record_offset":57976,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":58468,"parent_record_offset":58412,"ancestor_record_offset":57976,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":58380,"parent_record_offset":58348,"ancestor_record_offset":57976,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":58220,"parent_record_offset":58176,"ancestor_record_offset":57976,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":58492,"parent_record_offset":58468,"ancestor_record_offset":57976,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":58784,"parent_record_offset":58752,"ancestor_record_offset":57976,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":59068,"parent_record_offset":58784,"ancestor_record_offset":57976,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":59040,"parent_record_offset":58784,"ancestor_record_offset":57976,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":58844,"parent_record_offset":58784,"ancestor_record_offset":57976,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":60964,"parent_record_offset":60940,"ancestor_record_offset":57976,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":60988,"parent_record_offset":60964,"ancestor_record_offset":57976,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":62492,"parent_record_offset":62452,"ancestor_record_offset":57976,"inlinee":33645,"source_line":289,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":61128,"parent_record_offset":61104,"ancestor_record_offset":57976,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":61104,"parent_record_offset":61080,"ancestor_record_offset":57976,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":61152,"parent_record_offset":61128,"ancestor_record_offset":57976,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":59624,"parent_record_offset":59588,"ancestor_record_offset":57976,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":64036,"parent_record_offset":64004,"ancestor_record_offset":57976,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":64516,"parent_record_offset":64300,"ancestor_record_offset":57976,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1800,"record_offset":59780,"parent_record_offset":59752,"ancestor_record_offset":57976,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":59852,"parent_record_offset":59704,"ancestor_record_offset":57976,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":60068,"parent_record_offset":59852,"ancestor_record_offset":57976,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":60508,"parent_record_offset":60472,"ancestor_record_offset":57976,"inlinee":15004,"source_line":293,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$9::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":62524,"parent_record_offset":62452,"ancestor_record_offset":57976,"inlinee":29134,"source_line":674,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":62608,"parent_record_offset":62556,"ancestor_record_offset":57976,"inlinee":9136,"source_line":464,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":62556,"parent_record_offset":62524,"ancestor_record_offset":57976,"inlinee":9135,"source_line":528,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":63224,"parent_record_offset":62556,"ancestor_record_offset":57976,"inlinee":9138,"source_line":510,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1800,"record_offset":59808,"parent_record_offset":59780,"ancestor_record_offset":57976,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1800,"record_offset":59704,"parent_record_offset":59660,"ancestor_record_offset":57976,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1800,"record_offset":59660,"parent_record_offset":59624,"ancestor_record_offset":57976,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::CurrentSaveFile::change_section
// 记录形式: compiled_procedure; instances=1
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1800,"record_offset":16468,"rva":28766672,"va":5397475792}
// 编译实例 RVAs: 0x1b6f1d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21797817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e84764b3fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5397476500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-21798951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e8d95fb3fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397477650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28754300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e8843e49fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5397476487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048d580
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23993909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e8cbe191fe","size":5,"soft_fail":false,"target_va":5373482368,"va":5397476272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f5f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17760510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e802fff0fe","size":5,"soft_fail":false,"target_va":5379716592,"va":5397477097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141519000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6645257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e8f7999aff","size":5,"soft_fail":false,"target_va":5390831616,"va":5397476868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18875513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e879042001","size":5,"soft_fail":false,"target_va":5416353040,"va":5397477522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18907578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e8ba812001","size":5,"soft_fail":false,"target_va":5416385024,"va":5397477441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19580521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e869c62a01","size":5,"soft_fail":false,"target_va":5417057056,"va":5397476530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19580048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e890c42a01","size":5,"soft_fail":false,"target_va":5417057088,"va":5397477035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12682064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e8b07c3eff","size":5,"soft_fail":false,"target_va":5384794864,"va":5397476923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19142875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e8db182401","size":5,"soft_fail":false,"target_va":5416620794,"va":5397477914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19143612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e8bc1b2401","size":5,"soft_fail":false,"target_va":5416621408,"va":5397477791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19144975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e80f212401","size":5,"soft_fail":false,"target_va":5416621536,"va":5397476556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19143758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e84e1c2401","size":5,"soft_fail":false,"target_va":5416621188,"va":5397477425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19144823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e877202401","size":5,"soft_fail":false,"target_va":5416622320,"va":5397477492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t50448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e810c50000","size":5,"soft_fail":false,"target_va":5397526336,"va":5397475883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19143518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e85e1b2401","size":5,"soft_fail":false,"target_va":5416621376,"va":5397477853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13512438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e80ad131ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5397476225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13503934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e842f231ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5397476153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13512844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e874cf31ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5397477751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13513035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e8b5ce31ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5397475846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19141537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397475792,"raw_hex":"e8a1132401","size":5,"soft_fail":false,"target_va":5416619360,"va":5397477818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1800,"record_offset":21420,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":20040,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1800,"record_offset":21392,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":7362,"source_line":529,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":22380,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":20512,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1800,"record_offset":21504,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1800,"record_offset":20816,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":23276,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1800,"record_offset":19972,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":21324,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":19944,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":33689,"source_line":112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFileWriter::write_all
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":21532,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":33697,"source_line":281,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":18096,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":16696,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":17084,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1800,"record_offset":20540,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":20111,"source_line":576,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1800,"record_offset":18904,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$55::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1800,"record_offset":20844,"parent_record_offset":16468,"ancestor_record_offset":16468,"inlinee":21910,"source_line":2271,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":20336,"parent_record_offset":20312,"ancestor_record_offset":16468,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":20360,"parent_record_offset":20336,"ancestor_record_offset":16468,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":23452,"parent_record_offset":23428,"ancestor_record_offset":16468,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":23476,"parent_record_offset":23452,"ancestor_record_offset":16468,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":20384,"parent_record_offset":20360,"ancestor_record_offset":16468,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":23500,"parent_record_offset":23476,"ancestor_record_offset":16468,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":20312,"parent_record_offset":20276,"ancestor_record_offset":16468,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":23428,"parent_record_offset":23372,"ancestor_record_offset":16468,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1800,"record_offset":21468,"parent_record_offset":21444,"ancestor_record_offset":16468,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":21744,"parent_record_offset":21720,"ancestor_record_offset":16468,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":22072,"parent_record_offset":22040,"ancestor_record_offset":16468,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":21860,"parent_record_offset":21832,"ancestor_record_offset":16468,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":20208,"parent_record_offset":20176,"ancestor_record_offset":16468,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":21768,"parent_record_offset":21744,"ancestor_record_offset":16468,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":23400,"parent_record_offset":23372,"ancestor_record_offset":16468,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":23372,"parent_record_offset":23348,"ancestor_record_offset":16468,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":21920,"parent_record_offset":21888,"ancestor_record_offset":16468,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":22120,"parent_record_offset":22096,"ancestor_record_offset":16468,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":22096,"parent_record_offset":22072,"ancestor_record_offset":16468,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":21888,"parent_record_offset":21860,"ancestor_record_offset":16468,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":20276,"parent_record_offset":20240,"ancestor_record_offset":16468,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":20240,"parent_record_offset":20208,"ancestor_record_offset":16468,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":23348,"parent_record_offset":23324,"ancestor_record_offset":16468,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":20072,"parent_record_offset":20040,"ancestor_record_offset":16468,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":20104,"parent_record_offset":20072,"ancestor_record_offset":16468,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":20136,"parent_record_offset":20104,"ancestor_record_offset":16468,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":22040,"parent_record_offset":22016,"ancestor_record_offset":16468,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":21720,"parent_record_offset":21640,"ancestor_record_offset":16468,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":21804,"parent_record_offset":21640,"ancestor_record_offset":16468,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":21832,"parent_record_offset":21640,"ancestor_record_offset":16468,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":22220,"parent_record_offset":21640,"ancestor_record_offset":16468,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":22016,"parent_record_offset":21640,"ancestor_record_offset":16468,"inlinee":9137,"source_line":3195,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":20176,"parent_record_offset":20136,"ancestor_record_offset":16468,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":18424,"parent_record_offset":18396,"ancestor_record_offset":16468,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":18452,"parent_record_offset":18424,"ancestor_record_offset":16468,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":18372,"parent_record_offset":18344,"ancestor_record_offset":16468,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":18396,"parent_record_offset":18372,"ancestor_record_offset":16468,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":18512,"parent_record_offset":18396,"ancestor_record_offset":16468,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":18624,"parent_record_offset":18600,"ancestor_record_offset":16468,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":19700,"parent_record_offset":19676,"ancestor_record_offset":16468,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1800,"record_offset":20732,"parent_record_offset":20708,"ancestor_record_offset":16468,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1800,"record_offset":19268,"parent_record_offset":19244,"ancestor_record_offset":16468,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1800,"record_offset":19352,"parent_record_offset":19328,"ancestor_record_offset":16468,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1800,"record_offset":19768,"parent_record_offset":19132,"ancestor_record_offset":16468,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1800,"record_offset":18476,"parent_record_offset":18452,"ancestor_record_offset":16468,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":21692,"parent_record_offset":21640,"ancestor_record_offset":16468,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":21944,"parent_record_offset":21920,"ancestor_record_offset":16468,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1800,"record_offset":19500,"parent_record_offset":19472,"ancestor_record_offset":16468,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1800,"record_offset":19472,"parent_record_offset":19444,"ancestor_record_offset":16468,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1800,"record_offset":18244,"parent_record_offset":18204,"ancestor_record_offset":16468,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":21444,"parent_record_offset":21420,"ancestor_record_offset":16468,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":17032,"parent_record_offset":17004,"ancestor_record_offset":16468,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":18860,"parent_record_offset":18204,"ancestor_record_offset":16468,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":20440,"parent_record_offset":20136,"ancestor_record_offset":16468,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":22304,"parent_record_offset":22276,"ancestor_record_offset":16468,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":19292,"parent_record_offset":19268,"ancestor_record_offset":16468,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":18576,"parent_record_offset":18344,"ancestor_record_offset":16468,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":22276,"parent_record_offset":22252,"ancestor_record_offset":16468,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":19616,"parent_record_offset":19592,"ancestor_record_offset":16468,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":17504,"parent_record_offset":17480,"ancestor_record_offset":16468,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":17480,"parent_record_offset":17456,"ancestor_record_offset":16468,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":17688,"parent_record_offset":17660,"ancestor_record_offset":16468,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":17788,"parent_record_offset":17760,"ancestor_record_offset":16468,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1800,"record_offset":17760,"parent_record_offset":17732,"ancestor_record_offset":16468,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1800,"record_offset":22164,"parent_record_offset":21640,"ancestor_record_offset":16468,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1800,"record_offset":20000,"parent_record_offset":19972,"ancestor_record_offset":16468,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":17432,"parent_record_offset":17408,"ancestor_record_offset":16468,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":17456,"parent_record_offset":17432,"ancestor_record_offset":16468,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":17816,"parent_record_offset":17788,"ancestor_record_offset":16468,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":17408,"parent_record_offset":17292,"ancestor_record_offset":16468,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":1800,"record_offset":22188,"parent_record_offset":22164,"ancestor_record_offset":16468,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1800,"record_offset":16824,"parent_record_offset":16800,"ancestor_record_offset":16468,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1800,"record_offset":17120,"parent_record_offset":17084,"ancestor_record_offset":16468,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1800,"record_offset":17260,"parent_record_offset":17228,"ancestor_record_offset":16468,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1800,"record_offset":17228,"parent_record_offset":17192,"ancestor_record_offset":16468,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":17168,"parent_record_offset":17144,"ancestor_record_offset":16468,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":17192,"parent_record_offset":17168,"ancestor_record_offset":16468,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":17144,"parent_record_offset":17120,"ancestor_record_offset":16468,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":17732,"parent_record_offset":17192,"ancestor_record_offset":16468,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":16800,"parent_record_offset":16768,"ancestor_record_offset":16468,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":16768,"parent_record_offset":16740,"ancestor_record_offset":16468,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":16924,"parent_record_offset":16892,"ancestor_record_offset":16468,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":16980,"parent_record_offset":16924,"ancestor_record_offset":16468,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":16892,"parent_record_offset":16864,"ancestor_record_offset":16468,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":16740,"parent_record_offset":16696,"ancestor_record_offset":16468,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":17004,"parent_record_offset":16980,"ancestor_record_offset":16468,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":17292,"parent_record_offset":17260,"ancestor_record_offset":16468,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":17576,"parent_record_offset":17292,"ancestor_record_offset":16468,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":17548,"parent_record_offset":17292,"ancestor_record_offset":16468,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":17352,"parent_record_offset":17292,"ancestor_record_offset":16468,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_game::impl$15::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_game.rs
//     evidence={"module":1800,"record_offset":19724,"parent_record_offset":19700,"ancestor_record_offset":16468,"inlinee":8761,"source_line":531,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":22448,"parent_record_offset":22412,"ancestor_record_offset":16468,"inlinee":33645,"source_line":289,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":19412,"parent_record_offset":19384,"ancestor_record_offset":16468,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":19444,"parent_record_offset":19412,"ancestor_record_offset":16468,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":19384,"parent_record_offset":19132,"ancestor_record_offset":16468,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":19244,"parent_record_offset":19132,"ancestor_record_offset":16468,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":20708,"parent_record_offset":20680,"ancestor_record_offset":16468,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":19796,"parent_record_offset":19132,"ancestor_record_offset":16468,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":19328,"parent_record_offset":19132,"ancestor_record_offset":16468,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1800,"record_offset":19188,"parent_record_offset":19132,"ancestor_record_offset":16468,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1800,"record_offset":19676,"parent_record_offset":19652,"ancestor_record_offset":16468,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":18968,"parent_record_offset":18932,"ancestor_record_offset":16468,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":20596,"parent_record_offset":20568,"ancestor_record_offset":16468,"inlinee":20113,"source_line":799,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":19652,"parent_record_offset":19544,"ancestor_record_offset":16468,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":20892,"parent_record_offset":20868,"ancestor_record_offset":16468,"inlinee":13411,"source_line":3267,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":19056,"parent_record_offset":18968,"ancestor_record_offset":16468,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":19592,"parent_record_offset":19568,"ancestor_record_offset":16468,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":21164,"parent_record_offset":20944,"ancestor_record_offset":16468,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":19888,"parent_record_offset":19132,"ancestor_record_offset":16468,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":20680,"parent_record_offset":20652,"ancestor_record_offset":16468,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":20944,"parent_record_offset":20916,"ancestor_record_offset":16468,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":19568,"parent_record_offset":19544,"ancestor_record_offset":16468,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":19108,"parent_record_offset":19084,"ancestor_record_offset":16468,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":19084,"parent_record_offset":18968,"ancestor_record_offset":16468,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":19000,"parent_record_offset":18968,"ancestor_record_offset":16468,"inlinee":9370,"source_line":1370,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":20624,"parent_record_offset":20596,"ancestor_record_offset":16468,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":19024,"parent_record_offset":19000,"ancestor_record_offset":16468,"inlinee":9371,"source_line":1364,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":19132,"parent_record_offset":19108,"ancestor_record_offset":16468,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":20652,"parent_record_offset":20624,"ancestor_record_offset":16468,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":19216,"parent_record_offset":19132,"ancestor_record_offset":16468,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":20916,"parent_record_offset":20892,"ancestor_record_offset":16468,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":19544,"parent_record_offset":19132,"ancestor_record_offset":16468,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":18132,"parent_record_offset":18096,"ancestor_record_offset":16468,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":24164,"parent_record_offset":24132,"ancestor_record_offset":16468,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":24604,"parent_record_offset":24400,"ancestor_record_offset":16468,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1800,"record_offset":18272,"parent_record_offset":18244,"ancestor_record_offset":16468,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":18344,"parent_record_offset":18204,"ancestor_record_offset":16468,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":18548,"parent_record_offset":18344,"ancestor_record_offset":16468,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1800,"record_offset":18932,"parent_record_offset":18904,"ancestor_record_offset":16468,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1800,"record_offset":20568,"parent_record_offset":20540,"ancestor_record_offset":16468,"inlinee":20112,"source_line":673,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$41::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1800,"record_offset":20868,"parent_record_offset":20844,"ancestor_record_offset":16468,"inlinee":21911,"source_line":2113,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$9::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":21568,"parent_record_offset":21532,"ancestor_record_offset":16468,"inlinee":29134,"source_line":674,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":21640,"parent_record_offset":21596,"ancestor_record_offset":16468,"inlinee":9136,"source_line":464,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":21596,"parent_record_offset":21568,"ancestor_record_offset":16468,"inlinee":9135,"source_line":528,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":22252,"parent_record_offset":21596,"ancestor_record_offset":16468,"inlinee":9138,"source_line":510,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1800,"record_offset":18300,"parent_record_offset":18272,"ancestor_record_offset":16468,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1800,"record_offset":18204,"parent_record_offset":18168,"ancestor_record_offset":16468,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1800,"record_offset":18168,"parent_record_offset":18132,"ancestor_record_offset":16468,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync
// 记录形式: compiled_procedure; instances=1
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1800,"record_offset":26520,"rva":28769376,"va":5397478496}
// 编译实例 RVAs: 0x1b6fc60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-21800319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e8815ab3fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397479018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4700111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e83148b8ff","size":5,"soft_fail":false,"target_va":5392780768,"va":5397480874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18904550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e8e6752001","size":5,"soft_fail":false,"target_va":5416385024,"va":5397480469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19140944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e850112401","size":5,"soft_fail":false,"target_va":5416621408,"va":5397480459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t47768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e898ba0000","size":5,"soft_fail":false,"target_va":5397526336,"va":5397478563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19140725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e875102401","size":5,"soft_fail":false,"target_va":5416621376,"va":5397480646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::CurrentSaveFile::change_section
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e878f3ffff","size":5,"soft_fail":false,"target_va":5397475792,"va":5397478995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::shadow_file::ShadowFile::sync_all
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17755593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e83712f1fe","size":5,"soft_fail":false,"target_va":5379724720,"va":5397480308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13515117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e893c631ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5397478904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13506613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e8cbe731ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5397478832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13515459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e83dc531ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5397480366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13515719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e839c431ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5397478530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19138848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397478496,"raw_hex":"e820092401","size":5,"soft_fail":false,"target_va":5416619360,"va":5397480507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":28904,"parent_record_offset":26520,"ancestor_record_offset":26520,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":29108,"parent_record_offset":26520,"ancestor_record_offset":26520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":28096,"parent_record_offset":26520,"ancestor_record_offset":26520,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":26704,"parent_record_offset":26520,"ancestor_record_offset":26520,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":27084,"parent_record_offset":26520,"ancestor_record_offset":26520,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":29740,"parent_record_offset":29668,"ancestor_record_offset":26520,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":29812,"parent_record_offset":29740,"ancestor_record_offset":26520,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":29884,"parent_record_offset":29812,"ancestor_record_offset":26520,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":29668,"parent_record_offset":29468,"ancestor_record_offset":26520,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":29592,"parent_record_offset":29468,"ancestor_record_offset":26520,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":29468,"parent_record_offset":29396,"ancestor_record_offset":26520,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":29396,"parent_record_offset":29324,"ancestor_record_offset":26520,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":28424,"parent_record_offset":28396,"ancestor_record_offset":26520,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":28452,"parent_record_offset":28424,"ancestor_record_offset":26520,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":28372,"parent_record_offset":28344,"ancestor_record_offset":26520,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":28396,"parent_record_offset":28372,"ancestor_record_offset":26520,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":28512,"parent_record_offset":28396,"ancestor_record_offset":26520,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":28624,"parent_record_offset":28600,"ancestor_record_offset":26520,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1800,"record_offset":28476,"parent_record_offset":28452,"ancestor_record_offset":26520,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1800,"record_offset":28244,"parent_record_offset":28204,"ancestor_record_offset":26520,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":27032,"parent_record_offset":27008,"ancestor_record_offset":26520,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":28860,"parent_record_offset":28204,"ancestor_record_offset":26520,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":28576,"parent_record_offset":28344,"ancestor_record_offset":26520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":27504,"parent_record_offset":27480,"ancestor_record_offset":26520,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":27480,"parent_record_offset":27456,"ancestor_record_offset":26520,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":27688,"parent_record_offset":27660,"ancestor_record_offset":26520,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":27788,"parent_record_offset":27760,"ancestor_record_offset":26520,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1800,"record_offset":27760,"parent_record_offset":27732,"ancestor_record_offset":26520,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":27432,"parent_record_offset":27408,"ancestor_record_offset":26520,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":27456,"parent_record_offset":27432,"ancestor_record_offset":26520,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":27816,"parent_record_offset":27788,"ancestor_record_offset":26520,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":27408,"parent_record_offset":27292,"ancestor_record_offset":26520,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1800,"record_offset":26832,"parent_record_offset":26808,"ancestor_record_offset":26520,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1800,"record_offset":27120,"parent_record_offset":27084,"ancestor_record_offset":26520,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1800,"record_offset":27260,"parent_record_offset":27228,"ancestor_record_offset":26520,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1800,"record_offset":27228,"parent_record_offset":27192,"ancestor_record_offset":26520,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":27168,"parent_record_offset":27144,"ancestor_record_offset":26520,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":26520,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":27144,"parent_record_offset":27120,"ancestor_record_offset":26520,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1800,"record_offset":27732,"parent_record_offset":27192,"ancestor_record_offset":26520,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":26808,"parent_record_offset":26776,"ancestor_record_offset":26520,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":26776,"parent_record_offset":26748,"ancestor_record_offset":26520,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":26932,"parent_record_offset":26900,"ancestor_record_offset":26520,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":26984,"parent_record_offset":26932,"ancestor_record_offset":26520,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":26900,"parent_record_offset":26872,"ancestor_record_offset":26520,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":26748,"parent_record_offset":26704,"ancestor_record_offset":26520,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":27008,"parent_record_offset":26984,"ancestor_record_offset":26520,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":27292,"parent_record_offset":27260,"ancestor_record_offset":26520,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":27576,"parent_record_offset":27292,"ancestor_record_offset":26520,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":27548,"parent_record_offset":27292,"ancestor_record_offset":26520,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1800,"record_offset":27352,"parent_record_offset":27292,"ancestor_record_offset":26520,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":28132,"parent_record_offset":28096,"ancestor_record_offset":26520,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":31908,"parent_record_offset":31876,"ancestor_record_offset":26520,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":32380,"parent_record_offset":32164,"ancestor_record_offset":26520,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1800,"record_offset":28272,"parent_record_offset":28244,"ancestor_record_offset":26520,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":28344,"parent_record_offset":28204,"ancestor_record_offset":26520,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1800,"record_offset":28548,"parent_record_offset":28344,"ancestor_record_offset":26520,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1800,"record_offset":28300,"parent_record_offset":28272,"ancestor_record_offset":26520,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1800,"record_offset":28204,"parent_record_offset":28168,"ancestor_record_offset":26520,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1800,"record_offset":28168,"parent_record_offset":28132,"ancestor_record_offset":26520,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::CurrentSaveFile::backup_save
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15670
// 首条位置: {"module":1800,"inlinee":87664}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1800,"record_offset":46684,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":51132,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":51324,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":48848,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":48708,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1800,"record_offset":46768,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1800,"record_offset":51068,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":51500,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":46316,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":1800,"record_offset":50604,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":36869,"source_line":54,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":48764,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":33689,"source_line":112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":51696,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":48624,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":50152,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":47428,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":51820,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":48792,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":46288,"parent_record_offset":46000,"ancestor_record_offset":46000,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":46564,"parent_record_offset":46540,"ancestor_record_offset":46000,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":46588,"parent_record_offset":46564,"ancestor_record_offset":46000,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":46612,"parent_record_offset":46588,"ancestor_record_offset":46000,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":46540,"parent_record_offset":46484,"ancestor_record_offset":46000,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1800,"record_offset":46732,"parent_record_offset":46708,"ancestor_record_offset":46000,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":51396,"parent_record_offset":51352,"ancestor_record_offset":46000,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":46512,"parent_record_offset":46484,"ancestor_record_offset":46000,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":46484,"parent_record_offset":46460,"ancestor_record_offset":46000,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":51452,"parent_record_offset":51424,"ancestor_record_offset":46000,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":51424,"parent_record_offset":51396,"ancestor_record_offset":46000,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":46460,"parent_record_offset":46436,"ancestor_record_offset":46000,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":51168,"parent_record_offset":51132,"ancestor_record_offset":46000,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":51352,"parent_record_offset":51324,"ancestor_record_offset":46000,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":51096,"parent_record_offset":51068,"ancestor_record_offset":46000,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1800,"record_offset":48928,"parent_record_offset":48884,"ancestor_record_offset":46000,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":48884,"parent_record_offset":48848,"ancestor_record_offset":46000,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":50728,"parent_record_offset":50604,"ancestor_record_offset":46000,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1800,"record_offset":47464,"parent_record_offset":47428,"ancestor_record_offset":46000,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1800,"record_offset":51248,"parent_record_offset":51204,"ancestor_record_offset":46000,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":48732,"parent_record_offset":48708,"ancestor_record_offset":46000,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":46708,"parent_record_offset":46684,"ancestor_record_offset":46000,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=102
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":46340,"parent_record_offset":46316,"ancestor_record_offset":46000,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":50684,"parent_record_offset":50648,"ancestor_record_offset":46000,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1800,"record_offset":51204,"parent_record_offset":51168,"ancestor_record_offset":46000,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":48648,"parent_record_offset":48624,"ancestor_record_offset":46000,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":48672,"parent_record_offset":48648,"ancestor_record_offset":46000,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1800,"record_offset":50176,"parent_record_offset":50152,"ancestor_record_offset":46000,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":51752,"parent_record_offset":51724,"ancestor_record_offset":46000,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":51724,"parent_record_offset":51696,"ancestor_record_offset":46000,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":51780,"parent_record_offset":51752,"ancestor_record_offset":46000,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":50648,"parent_record_offset":50604,"ancestor_record_offset":46000,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$0::backup_save::closure$1
// 记录形式: inlinee_declaration; instances=1
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8871
// 首条位置: {"module":558,"inlinee":34929}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$0::backup_save::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B4F
// 首条位置: {"module":528,"inlinee":31567}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::time::impl$34::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":528,"record_offset":42228,"parent_record_offset":42204,"ancestor_record_offset":42204,"inlinee":31569,"source_line":248,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::SystemTime::intervals
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":528,"record_offset":42276,"parent_record_offset":42252,"ancestor_record_offset":42204,"inlinee":20849,"source_line":84,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$5::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":528,"record_offset":42252,"parent_record_offset":42228,"ancestor_record_offset":42204,"inlinee":31549,"source_line":129,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::CurrentSaveFile::perform_manual_backup
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15672
// 首条位置: {"module":1800,"inlinee":87666}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":71020,"parent_record_offset":70468,"ancestor_record_offset":70468,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":70568,"parent_record_offset":70468,"ancestor_record_offset":70468,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":71076,"parent_record_offset":70468,"ancestor_record_offset":70468,"inlinee":33689,"source_line":112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":70936,"parent_record_offset":70468,"ancestor_record_offset":70468,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":71104,"parent_record_offset":70468,"ancestor_record_offset":70468,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":70540,"parent_record_offset":70468,"ancestor_record_offset":70468,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":70816,"parent_record_offset":70792,"ancestor_record_offset":70468,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":70840,"parent_record_offset":70816,"ancestor_record_offset":70468,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":70864,"parent_record_offset":70840,"ancestor_record_offset":70468,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":70792,"parent_record_offset":70736,"ancestor_record_offset":70468,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":70764,"parent_record_offset":70736,"ancestor_record_offset":70468,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":70736,"parent_record_offset":70712,"ancestor_record_offset":70468,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":70712,"parent_record_offset":70688,"ancestor_record_offset":70468,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":71044,"parent_record_offset":71020,"ancestor_record_offset":70468,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":70592,"parent_record_offset":70568,"ancestor_record_offset":70468,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":70960,"parent_record_offset":70936,"ancestor_record_offset":70468,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":70984,"parent_record_offset":70960,"ancestor_record_offset":70468,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$0::perform_manual_backup::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 273
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112C9
// 首条位置: {"module":1253,"inlinee":70345}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":32056,"parent_record_offset":32032,"ancestor_record_offset":32032,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":32104,"parent_record_offset":32080,"ancestor_record_offset":32032,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1253,"record_offset":32080,"parent_record_offset":32056,"ancestor_record_offset":32032,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::UnsavedFileList::add
// 记录形式: compiled_procedure; instances=1
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1800,"record_offset":35196,"rva":28772544,"va":5397481664}
// 编译实例 RVAs: 0x1b708c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28176492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481664,"raw_hex":"e8940f52fe","size":5,"soft_fail":false,"target_va":5369305584,"va":5397482071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099a710
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18703146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481664,"raw_hex":"e8d69ce2fe","size":5,"soft_fail":false,"target_va":5378778896,"va":5397482037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18868454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481664,"raw_hex":"e8e6e81f01","size":5,"soft_fail":false,"target_va":5416350272,"va":5397481813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19094865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481664,"raw_hex":"e8515d2301","size":5,"soft_fail":false,"target_va":5416576832,"va":5397481962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19094157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481664,"raw_hex":"e88d5a2301","size":5,"soft_fail":false,"target_va":5416576144,"va":5397481982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13496875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481664,"raw_hex":"e8d50d32ff","size":5,"soft_fail":false,"target_va":5383984848,"va":5397481718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13496567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481664,"raw_hex":"e8090f32ff","size":5,"soft_fail":false,"target_va":5383985360,"va":5397481922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":35508,"parent_record_offset":35196,"ancestor_record_offset":35196,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":35744,"parent_record_offset":35196,"ancestor_record_offset":35196,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":1800,"record_offset":35352,"parent_record_offset":35196,"ancestor_record_offset":35196,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":35620,"parent_record_offset":35588,"ancestor_record_offset":35196,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":35668,"parent_record_offset":35644,"ancestor_record_offset":35196,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":35644,"parent_record_offset":35620,"ancestor_record_offset":35196,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":35588,"parent_record_offset":35540,"ancestor_record_offset":35196,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":35540,"parent_record_offset":35508,"ancestor_record_offset":35196,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":35708,"parent_record_offset":35540,"ancestor_record_offset":35196,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":35864,"parent_record_offset":35800,"ancestor_record_offset":35196,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":35416,"parent_record_offset":35380,"ancestor_record_offset":35196,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":35888,"parent_record_offset":35864,"ancestor_record_offset":35196,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":35440,"parent_record_offset":35416,"ancestor_record_offset":35196,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":1800,"record_offset":35772,"parent_record_offset":35744,"ancestor_record_offset":35196,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1800,"record_offset":35380,"parent_record_offset":35352,"ancestor_record_offset":35196,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1800,"record_offset":35800,"parent_record_offset":35772,"ancestor_record_offset":35196,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1800,"record_offset":35472,"parent_record_offset":35380,"ancestor_record_offset":35196,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1800,"record_offset":35836,"parent_record_offset":35800,"ancestor_record_offset":35196,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::UnsavedFileList::contains_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 300
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1800,"record_offset":34152,"rva":28772064,"va":5397481184}
// 编译实例 RVAs: 0x1b706e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::components
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12828157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481184,"raw_hex":"e803423cff","size":5,"soft_fail":false,"target_va":5384653152,"va":5397481304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28176056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481184,"raw_hex":"e8481152fe","size":5,"soft_fail":false,"target_va":5369305584,"va":5397481635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b1d970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17116833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481184,"raw_hex":"e85fd1fafe","size":5,"soft_fail":false,"target_va":5380364656,"va":5397481484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19575646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481184,"raw_hex":"e85eb32a01","size":5,"soft_fail":false,"target_va":5417057040,"va":5397481389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19095261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481184,"raw_hex":"e8dd5e2301","size":5,"soft_fail":false,"target_va":5416576832,"va":5397481566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19094558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481184,"raw_hex":"e81e5c2301","size":5,"soft_fail":false,"target_va":5416576144,"va":5397481581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13496405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481184,"raw_hex":"e8ab0f32ff","size":5,"soft_fail":false,"target_va":5383984848,"va":5397481248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13496161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397481184,"raw_hex":"e89f1032ff","size":5,"soft_fail":false,"target_va":5383985360,"va":5397481516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":35004,"parent_record_offset":34152,"ancestor_record_offset":34152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":34472,"parent_record_offset":34152,"ancestor_record_offset":34152,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":1800,"record_offset":34316,"parent_record_offset":34152,"ancestor_record_offset":34152,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":34612,"parent_record_offset":34560,"ancestor_record_offset":34152,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1800,"record_offset":34924,"parent_record_offset":34896,"ancestor_record_offset":34152,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1800,"record_offset":34896,"parent_record_offset":34660,"ancestor_record_offset":34152,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":34752,"parent_record_offset":34728,"ancestor_record_offset":34152,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":34528,"parent_record_offset":34504,"ancestor_record_offset":34152,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":34836,"parent_record_offset":34812,"ancestor_record_offset":34152,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":34860,"parent_record_offset":34836,"ancestor_record_offset":34152,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":34504,"parent_record_offset":34472,"ancestor_record_offset":34152,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":35124,"parent_record_offset":35060,"ancestor_record_offset":34152,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":34380,"parent_record_offset":34344,"ancestor_record_offset":34152,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":35148,"parent_record_offset":35124,"ancestor_record_offset":34152,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":34404,"parent_record_offset":34380,"ancestor_record_offset":34152,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::impl$1::contains_file::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":34560,"parent_record_offset":34472,"ancestor_record_offset":34152,"inlinee":87662,"source_line":301,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFile::path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":34584,"parent_record_offset":34560,"ancestor_record_offset":34152,"inlinee":33690,"source_line":247,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":1800,"record_offset":35032,"parent_record_offset":35004,"ancestor_record_offset":34152,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1800,"record_offset":34344,"parent_record_offset":34316,"ancestor_record_offset":34152,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1800,"record_offset":35060,"parent_record_offset":35032,"ancestor_record_offset":34152,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1800,"record_offset":34436,"parent_record_offset":34344,"ancestor_record_offset":34152,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1800,"record_offset":35096,"parent_record_offset":35060,"ancestor_record_offset":34152,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":34728,"parent_record_offset":34660,"ancestor_record_offset":34152,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":34700,"parent_record_offset":34660,"ancestor_record_offset":34152,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":34660,"parent_record_offset":34636,"ancestor_record_offset":34152,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$76::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":34636,"parent_record_offset":34612,"ancestor_record_offset":34152,"inlinee":34013,"source_line":3652,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$1::contains_file::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1566E
// 首条位置: {"module":1800,"inlinee":87662}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":34612,"parent_record_offset":34560,"ancestor_record_offset":34560,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1800,"record_offset":34584,"parent_record_offset":34560,"ancestor_record_offset":34560,"inlinee":33690,"source_line":247,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":34812,"parent_record_offset":34660,"ancestor_record_offset":34560,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1800,"record_offset":34924,"parent_record_offset":34896,"ancestor_record_offset":34560,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1800,"record_offset":34896,"parent_record_offset":34660,"ancestor_record_offset":34560,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":34752,"parent_record_offset":34728,"ancestor_record_offset":34560,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":34836,"parent_record_offset":34812,"ancestor_record_offset":34560,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":34860,"parent_record_offset":34836,"ancestor_record_offset":34560,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":34728,"parent_record_offset":34660,"ancestor_record_offset":34560,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":34700,"parent_record_offset":34660,"ancestor_record_offset":34560,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":34660,"parent_record_offset":34636,"ancestor_record_offset":34560,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$76::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":34636,"parent_record_offset":34612,"ancestor_record_offset":34560,"inlinee":34013,"source_line":3652,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$5::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 305
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D6D
// 首条位置: {"module":1643,"inlinee":81261}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$8::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 305
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14159
// 首条位置: {"module":1662,"inlinee":82265}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1662,"record_offset":35036,"parent_record_offset":35008,"ancestor_record_offset":35008,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01662.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1662,"record_offset":35060,"parent_record_offset":35036,"ancestor_record_offset":35008,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/01662.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1662,"record_offset":35088,"parent_record_offset":35060,"ancestor_record_offset":35008,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/01662.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1662,"record_offset":35116,"parent_record_offset":35088,"ancestor_record_offset":35008,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01662.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1662,"record_offset":35144,"parent_record_offset":35116,"ancestor_record_offset":35008,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01662.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$2::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 312
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1800,"record_offset":1004,"rva":28761680,"va":5397470800}
// 编译实例 RVAs: 0x1b6de50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12676107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397470800,"raw_hex":"e8f5933eff","size":5,"soft_fail":false,"target_va":5384794864,"va":5397470966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5397470800,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5397470840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1800,"record_offset":1424,"parent_record_offset":1004,"ancestor_record_offset":1004,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] anyhow::Error::downcast_ref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1800,"record_offset":1156,"parent_record_offset":1004,"ancestor_record_offset":1004,"inlinee":81645,"source_line":590,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::Error::kind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":1800,"record_offset":1212,"parent_record_offset":1004,"ancestor_record_offset":1004,"inlinee":17357,"source_line":1005,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":1800,"record_offset":1392,"parent_record_offset":1004,"ancestor_record_offset":1004,"inlinee":8741,"source_line":230,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1800,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":1004,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::error::vtable
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1800,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":1004,"inlinee":15975,"source_line":1068,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":1448,"parent_record_offset":1424,"ancestor_record_offset":1004,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::data
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1800,"record_offset":1256,"parent_record_offset":1212,"ancestor_record_offset":1004,"inlinee":8739,"source_line":208,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1800,"record_offset":1288,"parent_record_offset":1256,"ancestor_record_offset":1004,"inlinee":5225,"source_line":244,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::kind_from_prim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1800,"record_offset":1324,"parent_record_offset":1288,"ancestor_record_offset":1004,"inlinee":8742,"source_line":290,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::io::error::windows::decode_error_kind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\io\error\windows.rs
//     evidence={"module":1800,"record_offset":1360,"parent_record_offset":1212,"ancestor_record_offset":1004,"inlinee":17334,"source_line":13,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$10::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1566B
// 首条位置: {"module":1800,"inlinee":87659}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$11::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15665
// 首条位置: {"module":1800,"inlinee":87653}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$13::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15667
// 首条位置: {"module":1800,"inlinee":87655}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1800,"record_offset":3496,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":3468,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::AutosaveErrorState::is_ok
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 328, 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15669
// 首条位置: {"module":1800,"inlinee":87657}
// 编译实例 RVAs: 0xcb3030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":3596,"parent_record_offset":3568,"ancestor_record_offset":3568,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::save_thread::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 369
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1643,"record_offset":144708,"rva":23004288,"va":5391713408}
// 编译实例 RVAs: 0x15f0480
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7059909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391713408,"raw_hex":"e83b4694ff","size":5,"soft_fail":false,"target_va":5384653728,"va":5391713632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7104192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391713408,"raw_hex":"e8409993ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5391713595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16035118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391713408,"raw_hex":"e8d2520bff","size":5,"soft_fail":false,"target_va":5375678704,"va":5391713817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9e510
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9773375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391713408,"raw_hex":"e8c1de6aff","size":5,"soft_fail":false,"target_va":5381940496,"va":5391713866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b6de50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5757150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391713408,"raw_hex":"e8ded85700","size":5,"soft_fail":false,"target_va":5397470800,"va":5391713645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24637804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391713408,"raw_hex":"e86cf17701","size":5,"soft_fail":false,"target_va":5416351504,"va":5391713695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::shadow_file::ShadowFile::sync_all
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11988731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391713408,"raw_hex":"e8051149ff","size":5,"soft_fail":false,"target_va":5379724720,"va":5391713446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":145216,"parent_record_offset":144708,"ancestor_record_offset":144708,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":144968,"parent_record_offset":144708,"ancestor_record_offset":144708,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1643,"record_offset":145188,"parent_record_offset":144708,"ancestor_record_offset":144708,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":145448,"parent_record_offset":144708,"ancestor_record_offset":144708,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1643,"record_offset":145132,"parent_record_offset":144708,"ancestor_record_offset":144708,"inlinee":33690,"source_line":247,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1643,"record_offset":145024,"parent_record_offset":144708,"ancestor_record_offset":144708,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1643,"record_offset":144884,"parent_record_offset":144708,"ancestor_record_offset":144708,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1643,"record_offset":145160,"parent_record_offset":144708,"ancestor_record_offset":144708,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":145696,"parent_record_offset":145672,"ancestor_record_offset":144708,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":145720,"parent_record_offset":145696,"ancestor_record_offset":144708,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":145744,"parent_record_offset":145720,"ancestor_record_offset":144708,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":145672,"parent_record_offset":145616,"ancestor_record_offset":144708,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":145324,"parent_record_offset":145292,"ancestor_record_offset":144708,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":145644,"parent_record_offset":145616,"ancestor_record_offset":144708,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":145616,"parent_record_offset":145592,"ancestor_record_offset":144708,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":145372,"parent_record_offset":145348,"ancestor_record_offset":144708,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":145348,"parent_record_offset":145324,"ancestor_record_offset":144708,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":145592,"parent_record_offset":145568,"ancestor_record_offset":144708,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":145292,"parent_record_offset":145244,"ancestor_record_offset":144708,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":145244,"parent_record_offset":145216,"ancestor_record_offset":144708,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1643,"record_offset":144992,"parent_record_offset":144968,"ancestor_record_offset":144708,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":145472,"parent_record_offset":145448,"ancestor_record_offset":144708,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":145412,"parent_record_offset":145244,"ancestor_record_offset":144708,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1643,"record_offset":144908,"parent_record_offset":144884,"ancestor_record_offset":144708,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1643,"record_offset":144932,"parent_record_offset":144908,"ancestor_record_offset":144708,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1643,"record_offset":145072,"parent_record_offset":145048,"ancestor_record_offset":144708,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1643,"record_offset":145048,"parent_record_offset":145024,"ancestor_record_offset":144708,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1643,"record_offset":145096,"parent_record_offset":145072,"ancestor_record_offset":144708,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::save_thread::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 427
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x140F2
// 首条位置: {"module":1658,"inlinee":82162}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1658,"record_offset":397264,"parent_record_offset":396004,"ancestor_record_offset":396004,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1658,"record_offset":396068,"parent_record_offset":396004,"ancestor_record_offset":396004,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1658,"record_offset":397320,"parent_record_offset":396004,"ancestor_record_offset":396004,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1658,"record_offset":397180,"parent_record_offset":396004,"ancestor_record_offset":396004,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1658,"record_offset":396096,"parent_record_offset":396004,"ancestor_record_offset":396004,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1658,"record_offset":397124,"parent_record_offset":396004,"ancestor_record_offset":396004,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1658,"record_offset":397148,"parent_record_offset":397124,"ancestor_record_offset":396004,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1658,"record_offset":396456,"parent_record_offset":396432,"ancestor_record_offset":396004,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1658,"record_offset":396540,"parent_record_offset":396516,"ancestor_record_offset":396004,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1658,"record_offset":396936,"parent_record_offset":396308,"ancestor_record_offset":396004,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1658,"record_offset":396692,"parent_record_offset":396664,"ancestor_record_offset":396004,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1658,"record_offset":396664,"parent_record_offset":396636,"ancestor_record_offset":396004,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1658,"record_offset":397288,"parent_record_offset":397264,"ancestor_record_offset":396004,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1658,"record_offset":396480,"parent_record_offset":396456,"ancestor_record_offset":396004,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1658,"record_offset":396812,"parent_record_offset":396788,"ancestor_record_offset":396004,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1658,"record_offset":397204,"parent_record_offset":397180,"ancestor_record_offset":396004,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1658,"record_offset":397228,"parent_record_offset":397204,"ancestor_record_offset":396004,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1658,"record_offset":396600,"parent_record_offset":396572,"ancestor_record_offset":396004,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1658,"record_offset":396636,"parent_record_offset":396600,"ancestor_record_offset":396004,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1658,"record_offset":396572,"parent_record_offset":396308,"ancestor_record_offset":396004,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":1658,"record_offset":396432,"parent_record_offset":396308,"ancestor_record_offset":396004,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":1658,"record_offset":396976,"parent_record_offset":396308,"ancestor_record_offset":396004,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":1658,"record_offset":396516,"parent_record_offset":396308,"ancestor_record_offset":396004,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/tag.rs
//     evidence={"module":1658,"record_offset":396376,"parent_record_offset":396308,"ancestor_record_offset":396004,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1658,"record_offset":396168,"parent_record_offset":396132,"ancestor_record_offset":396004,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1658,"record_offset":396904,"parent_record_offset":396736,"ancestor_record_offset":396004,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1658,"record_offset":396208,"parent_record_offset":396168,"ancestor_record_offset":396004,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1658,"record_offset":396844,"parent_record_offset":396760,"ancestor_record_offset":396004,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1658,"record_offset":396788,"parent_record_offset":396760,"ancestor_record_offset":396004,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":1658,"record_offset":397068,"parent_record_offset":396308,"ancestor_record_offset":396004,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1658,"record_offset":396760,"parent_record_offset":396736,"ancestor_record_offset":396004,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1658,"record_offset":396272,"parent_record_offset":396236,"ancestor_record_offset":396004,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1658,"record_offset":396236,"parent_record_offset":396168,"ancestor_record_offset":396004,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":1658,"record_offset":396308,"parent_record_offset":396272,"ancestor_record_offset":396004,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":1658,"record_offset":396404,"parent_record_offset":396308,"ancestor_record_offset":396004,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1658,"record_offset":396736,"parent_record_offset":396308,"ancestor_record_offset":396004,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1658,"record_offset":396132,"parent_record_offset":396096,"ancestor_record_offset":396004,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1658,"record_offset":397368,"parent_record_offset":397344,"ancestor_record_offset":396004,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1658,"record_offset":397344,"parent_record_offset":397320,"ancestor_record_offset":396004,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1658,"record_offset":397392,"parent_record_offset":397368,"ancestor_record_offset":396004,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01658.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::save_thread::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 499
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1566C
// 首条位置: {"module":1800,"inlinee":87660}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":9944,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":9992,"parent_record_offset":9968,"ancestor_record_offset":9944,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":1800,"record_offset":10072,"parent_record_offset":10048,"ancestor_record_offset":9944,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$20::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":1800,"record_offset":10048,"parent_record_offset":10024,"ancestor_record_offset":9944,"inlinee":6431,"source_line":13,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$24::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":1800,"record_offset":10024,"parent_record_offset":9968,"ancestor_record_offset":9944,"inlinee":6429,"source_line":154,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::execute_open_file_cmd
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 670
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15675
// 首条位置: {"module":1800,"inlinee":87669}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1800,"record_offset":97984,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":87068,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":91432,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":90616,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":84612,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1800,"record_offset":88424,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":11834,"source_line":225,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1800,"record_offset":94824,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":90472,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":84468,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":84668,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":99800,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::inspect_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":88000,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":11771,"source_line":1016,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$7::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":88328,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":33954,"source_line":1939,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_backup_list::SaveBackupList::into_files
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":1800,"record_offset":94796,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":36928,"source_line":136,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::full_write_history_file
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":92672,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":87671,"source_line":750,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":84528,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":87124,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":87096,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":1800,"record_offset":97636,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":7548,"source_line":519,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":1800,"record_offset":97956,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":7549,"source_line":78,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":1800,"record_offset":98528,"parent_record_offset":84156,"ancestor_record_offset":84156,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":97844,"parent_record_offset":97820,"ancestor_record_offset":84156,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":97868,"parent_record_offset":97844,"ancestor_record_offset":84156,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":85060,"parent_record_offset":85032,"ancestor_record_offset":84156,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":85088,"parent_record_offset":85060,"ancestor_record_offset":84156,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":97892,"parent_record_offset":97868,"ancestor_record_offset":84156,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":85120,"parent_record_offset":85088,"ancestor_record_offset":84156,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":97820,"parent_record_offset":97788,"ancestor_record_offset":84156,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":85032,"parent_record_offset":84844,"ancestor_record_offset":84156,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1800,"record_offset":97788,"parent_record_offset":97744,"ancestor_record_offset":84156,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1800,"record_offset":97744,"parent_record_offset":97708,"ancestor_record_offset":84156,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1800,"record_offset":99496,"parent_record_offset":99468,"ancestor_record_offset":84156,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":91544,"parent_record_offset":91512,"ancestor_record_offset":84156,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":98216,"parent_record_offset":98180,"ancestor_record_offset":84156,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":85284,"parent_record_offset":85256,"ancestor_record_offset":84156,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":85256,"parent_record_offset":85232,"ancestor_record_offset":84156,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":91592,"parent_record_offset":91568,"ancestor_record_offset":84156,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":91568,"parent_record_offset":91544,"ancestor_record_offset":84156,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":98292,"parent_record_offset":98252,"ancestor_record_offset":84156,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":98252,"parent_record_offset":98216,"ancestor_record_offset":84156,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":85232,"parent_record_offset":85208,"ancestor_record_offset":84156,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":98068,"parent_record_offset":98020,"ancestor_record_offset":84156,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":98104,"parent_record_offset":98068,"ancestor_record_offset":84156,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":98140,"parent_record_offset":98104,"ancestor_record_offset":84156,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1800,"record_offset":98020,"parent_record_offset":97984,"ancestor_record_offset":84156,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1800,"record_offset":89108,"parent_record_offset":89072,"ancestor_record_offset":84156,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1800,"record_offset":89072,"parent_record_offset":89048,"ancestor_record_offset":84156,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":91512,"parent_record_offset":91464,"ancestor_record_offset":84156,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":91464,"parent_record_offset":91432,"ancestor_record_offset":84156,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":98180,"parent_record_offset":98140,"ancestor_record_offset":84156,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1800,"record_offset":96120,"parent_record_offset":96092,"ancestor_record_offset":84156,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1800,"record_offset":93048,"parent_record_offset":93020,"ancestor_record_offset":84156,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":93020,"parent_record_offset":92984,"ancestor_record_offset":84156,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":92984,"parent_record_offset":92672,"ancestor_record_offset":84156,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":95448,"parent_record_offset":95420,"ancestor_record_offset":84156,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1800,"record_offset":95340,"parent_record_offset":95120,"ancestor_record_offset":84156,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":88152,"parent_record_offset":88044,"ancestor_record_offset":84156,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":90672,"parent_record_offset":90644,"ancestor_record_offset":84156,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1800,"record_offset":85800,"parent_record_offset":85776,"ancestor_record_offset":84156,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1800,"record_offset":95776,"parent_record_offset":95744,"ancestor_record_offset":84156,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\cloned.rs
//     evidence={"module":1800,"record_offset":95868,"parent_record_offset":95840,"ancestor_record_offset":84156,"inlinee":6736,"source_line":57,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1800,"record_offset":88792,"parent_record_offset":88724,"ancestor_record_offset":84156,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1800,"record_offset":88680,"parent_record_offset":88588,"ancestor_record_offset":84156,"inlinee":11834,"source_line":225,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1800,"record_offset":88724,"parent_record_offset":88680,"ancestor_record_offset":84156,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$46::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1800,"record_offset":88476,"parent_record_offset":88424,"ancestor_record_offset":84156,"inlinee":11836,"source_line":851,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1800,"record_offset":88588,"parent_record_offset":88536,"ancestor_record_offset":84156,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1800,"record_offset":88536,"parent_record_offset":88476,"ancestor_record_offset":84156,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1800,"record_offset":95840,"parent_record_offset":95812,"ancestor_record_offset":84156,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1800,"record_offset":95988,"parent_record_offset":95924,"ancestor_record_offset":84156,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1800,"record_offset":95812,"parent_record_offset":95680,"ancestor_record_offset":84156,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1800,"record_offset":96188,"parent_record_offset":96164,"ancestor_record_offset":84156,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1800,"record_offset":84492,"parent_record_offset":84468,"ancestor_record_offset":84156,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":84936,"parent_record_offset":84908,"ancestor_record_offset":84156,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":84964,"parent_record_offset":84936,"ancestor_record_offset":84156,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":96256,"parent_record_offset":95924,"ancestor_record_offset":84156,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":85316,"parent_record_offset":85284,"ancestor_record_offset":84156,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":95476,"parent_record_offset":95420,"ancestor_record_offset":84156,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1800,"record_offset":86020,"parent_record_offset":85996,"ancestor_record_offset":84156,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1800,"record_offset":96012,"parent_record_offset":95988,"ancestor_record_offset":84156,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1800,"record_offset":88872,"parent_record_offset":88792,"ancestor_record_offset":84156,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":88640,"parent_record_offset":88588,"ancestor_record_offset":84156,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":92816,"parent_record_offset":92672,"ancestor_record_offset":84156,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":90240,"parent_record_offset":90136,"ancestor_record_offset":84156,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":90264,"parent_record_offset":90240,"ancestor_record_offset":84156,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":84636,"parent_record_offset":84612,"ancestor_record_offset":84156,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":90068,"parent_record_offset":90044,"ancestor_record_offset":84156,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":95152,"parent_record_offset":95120,"ancestor_record_offset":84156,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":90644,"parent_record_offset":90616,"ancestor_record_offset":84156,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":85832,"parent_record_offset":85748,"ancestor_record_offset":84156,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1800,"record_offset":98452,"parent_record_offset":98140,"ancestor_record_offset":84156,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":98476,"parent_record_offset":98452,"ancestor_record_offset":84156,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=110
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":84692,"parent_record_offset":84668,"ancestor_record_offset":84156,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":92920,"parent_record_offset":92896,"ancestor_record_offset":84156,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":86288,"parent_record_offset":86264,"ancestor_record_offset":84156,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":95244,"parent_record_offset":95120,"ancestor_record_offset":84156,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":85000,"parent_record_offset":84876,"ancestor_record_offset":84156,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":95956,"parent_record_offset":95924,"ancestor_record_offset":84156,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":89140,"parent_record_offset":89108,"ancestor_record_offset":84156,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":95272,"parent_record_offset":95244,"ancestor_record_offset":84156,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":91632,"parent_record_offset":91464,"ancestor_record_offset":84156,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":87164,"parent_record_offset":87124,"ancestor_record_offset":84156,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":88356,"parent_record_offset":88328,"ancestor_record_offset":84156,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":90044,"parent_record_offset":89992,"ancestor_record_offset":84156,"inlinee":33993,"source_line":2079,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":98588,"parent_record_offset":98556,"ancestor_record_offset":84156,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$7::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":90376,"parent_record_offset":88476,"ancestor_record_offset":84156,"inlinee":33954,"source_line":1939,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":90744,"parent_record_offset":90720,"ancestor_record_offset":84156,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":90768,"parent_record_offset":90744,"ancestor_record_offset":84156,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1800,"record_offset":92872,"parent_record_offset":92672,"ancestor_record_offset":84156,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1800,"record_offset":92896,"parent_record_offset":92872,"ancestor_record_offset":84156,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1800,"record_offset":95924,"parent_record_offset":95896,"ancestor_record_offset":84156,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":90696,"parent_record_offset":90672,"ancestor_record_offset":84156,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":98936,"parent_record_offset":98900,"ancestor_record_offset":84156,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":84552,"parent_record_offset":84528,"ancestor_record_offset":84156,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":84576,"parent_record_offset":84552,"ancestor_record_offset":84156,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":89168,"parent_record_offset":89108,"ancestor_record_offset":84156,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":89200,"parent_record_offset":89072,"ancestor_record_offset":84156,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1800,"record_offset":90572,"parent_record_offset":90548,"ancestor_record_offset":84156,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::execute_open_file_cmd::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":88044,"parent_record_offset":88000,"ancestor_record_offset":84156,"inlinee":87670,"source_line":692,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":85964,"parent_record_offset":85936,"ancestor_record_offset":84156,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":85996,"parent_record_offset":85964,"ancestor_record_offset":84156,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":85936,"parent_record_offset":85896,"ancestor_record_offset":84156,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":85776,"parent_record_offset":85748,"ancestor_record_offset":84156,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":95716,"parent_record_offset":95680,"ancestor_record_offset":84156,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":95680,"parent_record_offset":95648,"ancestor_record_offset":84156,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":96212,"parent_record_offset":96188,"ancestor_record_offset":84156,"inlinee":12038,"source_line":4661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":86264,"parent_record_offset":85660,"ancestor_record_offset":84156,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":86116,"parent_record_offset":85896,"ancestor_record_offset":84156,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":85748,"parent_record_offset":85692,"ancestor_record_offset":84156,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":85896,"parent_record_offset":85868,"ancestor_record_offset":84156,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":95744,"parent_record_offset":95716,"ancestor_record_offset":84156,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":84876,"parent_record_offset":84844,"ancestor_record_offset":84156,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":85720,"parent_record_offset":85692,"ancestor_record_offset":84156,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":85660,"parent_record_offset":85600,"ancestor_record_offset":84156,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":84788,"parent_record_offset":84764,"ancestor_record_offset":84156,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":84844,"parent_record_offset":84788,"ancestor_record_offset":84156,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":84816,"parent_record_offset":84788,"ancestor_record_offset":84156,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":85692,"parent_record_offset":85660,"ancestor_record_offset":84156,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":84908,"parent_record_offset":84876,"ancestor_record_offset":84156,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":84764,"parent_record_offset":84740,"ancestor_record_offset":84156,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":85868,"parent_record_offset":85660,"ancestor_record_offset":84156,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1800,"record_offset":95648,"parent_record_offset":95616,"ancestor_record_offset":84156,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":88256,"parent_record_offset":88232,"ancestor_record_offset":84156,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":88208,"parent_record_offset":88044,"ancestor_record_offset":84156,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":88232,"parent_record_offset":88208,"ancestor_record_offset":84156,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":88280,"parent_record_offset":88256,"ancestor_record_offset":84156,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":88068,"parent_record_offset":88044,"ancestor_record_offset":84156,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":95588,"parent_record_offset":94860,"ancestor_record_offset":84156,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":95616,"parent_record_offset":94860,"ancestor_record_offset":84156,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":94860,"parent_record_offset":94824,"ancestor_record_offset":84156,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1800,"record_offset":90524,"parent_record_offset":90500,"ancestor_record_offset":84156,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1800,"record_offset":96064,"parent_record_offset":96036,"ancestor_record_offset":84156,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1800,"record_offset":94940,"parent_record_offset":94912,"ancestor_record_offset":84156,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1800,"record_offset":95204,"parent_record_offset":95180,"ancestor_record_offset":84156,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1800,"record_offset":95060,"parent_record_offset":95032,"ancestor_record_offset":84156,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1800,"record_offset":95420,"parent_record_offset":94996,"ancestor_record_offset":84156,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1800,"record_offset":94912,"parent_record_offset":94860,"ancestor_record_offset":84156,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::into_inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":93096,"parent_record_offset":92672,"ancestor_record_offset":84156,"inlinee":87672,"source_line":116,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":92788,"parent_record_offset":92672,"ancestor_record_offset":84156,"inlinee":23648,"source_line":98,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":1800,"record_offset":98788,"parent_record_offset":98764,"ancestor_record_offset":84156,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":96036,"parent_record_offset":96012,"ancestor_record_offset":84156,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":1800,"record_offset":97672,"parent_record_offset":97636,"ancestor_record_offset":84156,"inlinee":5896,"source_line":198,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\counter.rs
//     evidence={"module":1800,"record_offset":97708,"parent_record_offset":97672,"ancestor_record_offset":84156,"inlinee":5885,"source_line":20,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1800,"record_offset":90548,"parent_record_offset":90524,"ancestor_record_offset":84156,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1800,"record_offset":96092,"parent_record_offset":96064,"ancestor_record_offset":84156,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1800,"record_offset":95088,"parent_record_offset":95060,"ancestor_record_offset":84156,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1800,"record_offset":95120,"parent_record_offset":95088,"ancestor_record_offset":84156,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":1800,"record_offset":98556,"parent_record_offset":98528,"ancestor_record_offset":84156,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1800,"record_offset":94996,"parent_record_offset":94968,"ancestor_record_offset":84156,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1800,"record_offset":94968,"parent_record_offset":94940,"ancestor_record_offset":84156,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::execute_open_file_cmd::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 692
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15676
// 首条位置: {"module":1800,"inlinee":87670}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":88152,"parent_record_offset":88044,"ancestor_record_offset":88044,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":88208,"parent_record_offset":88044,"ancestor_record_offset":88044,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":88068,"parent_record_offset":88044,"ancestor_record_offset":88044,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":88176,"parent_record_offset":88152,"ancestor_record_offset":88044,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":88092,"parent_record_offset":88068,"ancestor_record_offset":88044,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":88116,"parent_record_offset":88092,"ancestor_record_offset":88044,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":88256,"parent_record_offset":88232,"ancestor_record_offset":88044,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":88232,"parent_record_offset":88208,"ancestor_record_offset":88044,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":88280,"parent_record_offset":88256,"ancestor_record_offset":88044,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::execute_open_file_cmd::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 731
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F6C
// 首条位置: {"module":535,"inlinee":32620}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::full_write_history_file
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 750
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15677
// 首条位置: {"module":1800,"inlinee":87671}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":92984,"parent_record_offset":92672,"ancestor_record_offset":92672,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":92816,"parent_record_offset":92672,"ancestor_record_offset":92672,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":93124,"parent_record_offset":92672,"ancestor_record_offset":92672,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1800,"record_offset":92872,"parent_record_offset":92672,"ancestor_record_offset":92672,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::into_inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":93096,"parent_record_offset":92672,"ancestor_record_offset":92672,"inlinee":87672,"source_line":116,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":92788,"parent_record_offset":92672,"ancestor_record_offset":92672,"inlinee":23648,"source_line":98,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":93372,"parent_record_offset":93348,"ancestor_record_offset":92672,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":93396,"parent_record_offset":93372,"ancestor_record_offset":92672,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":93420,"parent_record_offset":93396,"ancestor_record_offset":92672,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":93348,"parent_record_offset":93292,"ancestor_record_offset":92672,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":93320,"parent_record_offset":93292,"ancestor_record_offset":92672,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":93292,"parent_record_offset":93268,"ancestor_record_offset":92672,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":93268,"parent_record_offset":93244,"ancestor_record_offset":92672,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1800,"record_offset":93048,"parent_record_offset":93020,"ancestor_record_offset":92672,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":93020,"parent_record_offset":92984,"ancestor_record_offset":92672,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":93148,"parent_record_offset":93124,"ancestor_record_offset":92672,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":92920,"parent_record_offset":92896,"ancestor_record_offset":92672,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1800,"record_offset":92896,"parent_record_offset":92872,"ancestor_record_offset":92672,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::capture_snapshot
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 796
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15674
// 首条位置: {"module":1800,"inlinee":87668}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":78604,"parent_record_offset":77864,"ancestor_record_offset":77864,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":78180,"parent_record_offset":77864,"ancestor_record_offset":77864,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":77980,"parent_record_offset":77864,"ancestor_record_offset":77864,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":78760,"parent_record_offset":77864,"ancestor_record_offset":77864,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::HistorySnapshotId::file_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1800,"record_offset":78068,"parent_record_offset":77864,"ancestor_record_offset":77864,"inlinee":35957,"source_line":285,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::HistorySnapshotId::from_snapshot_blob
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1800,"record_offset":78012,"parent_record_offset":77864,"ancestor_record_offset":77864,"inlinee":35955,"source_line":281,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":78492,"parent_record_offset":77864,"ancestor_record_offset":77864,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1800,"record_offset":78040,"parent_record_offset":77864,"ancestor_record_offset":77864,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":78380,"parent_record_offset":78356,"ancestor_record_offset":77864,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":78404,"parent_record_offset":78380,"ancestor_record_offset":77864,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":78428,"parent_record_offset":78404,"ancestor_record_offset":77864,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":78356,"parent_record_offset":78300,"ancestor_record_offset":77864,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1800,"record_offset":78092,"parent_record_offset":78068,"ancestor_record_offset":77864,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1800,"record_offset":78140,"parent_record_offset":78116,"ancestor_record_offset":77864,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":78328,"parent_record_offset":78300,"ancestor_record_offset":77864,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":78300,"parent_record_offset":78276,"ancestor_record_offset":77864,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":78276,"parent_record_offset":78252,"ancestor_record_offset":77864,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1800,"record_offset":78684,"parent_record_offset":78640,"ancestor_record_offset":77864,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1800,"record_offset":78640,"parent_record_offset":78604,"ancestor_record_offset":77864,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1800,"record_offset":78544,"parent_record_offset":78492,"ancestor_record_offset":77864,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":78116,"parent_record_offset":78092,"ancestor_record_offset":77864,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=52
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":78204,"parent_record_offset":78180,"ancestor_record_offset":77864,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":78572,"parent_record_offset":78492,"ancestor_record_offset":77864,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":78516,"parent_record_offset":78492,"ancestor_record_offset":77864,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":1800,"record_offset":80352,"parent_record_offset":80328,"ancestor_record_offset":77864,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::snapshot_gc_thread
// 记录形式: compiled_procedure; instances=1
// 源码行: 820
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1800,"record_offset":116832,"rva":28811168,"va":5397520288}
// 编译实例 RVAs: 0x1b79fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::OpenOptions>::_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12817109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e82b6d3cff","size":5,"soft_fail":false,"target_va":5384703536,"va":5397520640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::extension
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12865424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e870b03bff","size":5,"soft_fail":false,"target_va":5384656912,"va":5397522331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12863681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e83fb73bff","size":5,"soft_fail":false,"target_va":5384657024,"va":5397520700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_stem
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12865154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e87eb13bff","size":5,"soft_fail":false,"target_va":5384657312,"va":5397522461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12875128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e8888a3bff","size":5,"soft_fail":false,"target_va":5384648448,"va":5397523571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12872361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e857953bff","size":5,"soft_fail":false,"target_va":5384648032,"va":5397520388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-12911502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e872fc3aff","size":5,"soft_fail":false,"target_va":5384609408,"va":5397520905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21842276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e89cb6b2fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5397520959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=19
//     evidence={"assembly":"\tcallq\t-21842670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e812b5b2fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397521369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28798757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e8db9048fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5397520944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007db70
//     owner=unresolved_namespace; count=3
//     target_aliases=27 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28300857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e8c72950fe","size":5,"soft_fail":false,"target_va":5369224048,"va":5397524900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=2
//     target_aliases=37 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28254790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e8badd50fe","size":5,"soft_fail":false,"target_va":5369270288,"va":5397525073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008d710
//     owner=unresolved_namespace; count=2
//     target_aliases=21 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28235550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e8e22851fe","size":5,"soft_fail":false,"target_va":5369288464,"va":5397524009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=5
//     target_aliases=3 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28182079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e8c1f951fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5397520922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28180947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e82dfe51fe","size":5,"soft_fail":false,"target_va":5369343504,"va":5397524446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24043743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e8211f91fe","size":5,"soft_fail":false,"target_va":5373479040,"va":5397522778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14091ba90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19261932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e81416dafe","size":5,"soft_fail":false,"target_va":5378259600,"va":5397521527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096f020
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18921132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e85449dffe","size":5,"soft_fail":false,"target_va":5378600992,"va":5397522119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14179c630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4054510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e81222c2ff","size":5,"soft_fail":false,"target_va":5393466928,"va":5397521433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c149e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t632659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e853a70900","size":5,"soft_fail":false,"target_va":5398153696,"va":5397521032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19096949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e875652301","size":5,"soft_fail":false,"target_va":5416620794,"va":5397523840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12692283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e8c5543eff","size":5,"soft_fail":false,"target_va":5384830080,"va":5397522358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::remove_file
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12800420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397520288,"raw_hex":"e85cae3cff","size":5,"soft_fail":false,"target_va":5384722736,"va":5397523151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*28636820(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5397520288,"raw_hex":"ff1594f6b401","size":6,"soft_fail":false,"va":5397524350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":117024,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":117080,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":117436,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1800,"record_offset":119004,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":118352,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":119144,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":11666,"source_line":646,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::nuke_unreferenced_shapshots
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1800,"record_offset":120008,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":87673,"source_line":871,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":117492,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":117352,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":119060,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":117236,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":7533,"source_line":570,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::io::buffered::bufreader::BufReader::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\buffered\bufreader.rs
//     evidence={"module":1800,"record_offset":117604,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":25566,"source_line":73,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":1800,"record_offset":119116,"parent_record_offset":116832,"ancestor_record_offset":116832,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":117844,"parent_record_offset":117820,"ancestor_record_offset":116832,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":117868,"parent_record_offset":117844,"ancestor_record_offset":116832,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":118556,"parent_record_offset":118532,"ancestor_record_offset":116832,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":118580,"parent_record_offset":118556,"ancestor_record_offset":116832,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":117892,"parent_record_offset":117868,"ancestor_record_offset":116832,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":118604,"parent_record_offset":118580,"ancestor_record_offset":116832,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":117820,"parent_record_offset":117792,"ancestor_record_offset":116832,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":118532,"parent_record_offset":118500,"ancestor_record_offset":116832,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1800,"record_offset":118500,"parent_record_offset":118472,"ancestor_record_offset":116832,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::new_uninit_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1800,"record_offset":117704,"parent_record_offset":117676,"ancestor_record_offset":116832,"inlinee":15755,"source_line":907,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":117732,"parent_record_offset":117704,"ancestor_record_offset":116832,"inlinee":15756,"source_line":125,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":119504,"parent_record_offset":119472,"ancestor_record_offset":116832,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":119472,"parent_record_offset":119444,"ancestor_record_offset":116832,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":117792,"parent_record_offset":117760,"ancestor_record_offset":116832,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":117760,"parent_record_offset":117732,"ancestor_record_offset":116832,"inlinee":15757,"source_line":145,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":119444,"parent_record_offset":119416,"ancestor_record_offset":116832,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":120112,"parent_record_offset":120008,"ancestor_record_offset":116832,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":127536,"parent_record_offset":127512,"ancestor_record_offset":116832,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":119224,"parent_record_offset":119200,"ancestor_record_offset":116832,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":119708,"parent_record_offset":119684,"ancestor_record_offset":116832,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":120168,"parent_record_offset":120008,"ancestor_record_offset":116832,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::to_digit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":1800,"record_offset":121372,"parent_record_offset":121272,"ancestor_record_offset":116832,"inlinee":6771,"source_line":402,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":120460,"parent_record_offset":120008,"ancestor_record_offset":116832,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":122652,"parent_record_offset":120008,"ancestor_record_offset":116832,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":120516,"parent_record_offset":120488,"ancestor_record_offset":116832,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1800,"record_offset":121892,"parent_record_offset":121868,"ancestor_record_offset":116832,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1800,"record_offset":121976,"parent_record_offset":121952,"ancestor_record_offset":116832,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1800,"record_offset":122392,"parent_record_offset":121756,"ancestor_record_offset":116832,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1800,"record_offset":121488,"parent_record_offset":121432,"ancestor_record_offset":116832,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1800,"record_offset":119028,"parent_record_offset":119004,"ancestor_record_offset":116832,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::can_not_overflow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1800,"record_offset":121344,"parent_record_offset":121272,"ancestor_record_offset":116832,"inlinee":6770,"source_line":1521,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":124920,"parent_record_offset":124892,"ancestor_record_offset":116832,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":119836,"parent_record_offset":119808,"ancestor_record_offset":116832,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$40::from_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1800,"record_offset":121224,"parent_record_offset":121200,"ancestor_record_offset":116832,"inlinee":6859,"source_line":1578,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$41::from_ascii_radix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1800,"record_offset":121272,"parent_record_offset":121248,"ancestor_record_offset":116832,"inlinee":6862,"source_line":1708,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$41::from_str_radix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1800,"record_offset":121248,"parent_record_offset":121224,"ancestor_record_offset":116832,"inlinee":6861,"source_line":1629,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":121400,"parent_record_offset":121372,"ancestor_record_offset":116832,"inlinee":4145,"source_line":2493,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":121432,"parent_record_offset":121272,"ancestor_record_offset":116832,"inlinee":6867,"source_line":1160,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":121460,"parent_record_offset":121432,"ancestor_record_offset":116832,"inlinee":6868,"source_line":3050,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1800,"record_offset":122124,"parent_record_offset":122096,"ancestor_record_offset":116832,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1800,"record_offset":122096,"parent_record_offset":122068,"ancestor_record_offset":116832,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1800,"record_offset":120344,"parent_record_offset":120316,"ancestor_record_offset":116832,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":120316,"parent_record_offset":120008,"ancestor_record_offset":116832,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":117460,"parent_record_offset":117436,"ancestor_record_offset":116832,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":120488,"parent_record_offset":120460,"ancestor_record_offset":116832,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":121916,"parent_record_offset":121892,"ancestor_record_offset":116832,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=88
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":117952,"parent_record_offset":117640,"ancestor_record_offset":116832,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":117048,"parent_record_offset":117024,"ancestor_record_offset":116832,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":122240,"parent_record_offset":122216,"ancestor_record_offset":116832,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":117148,"parent_record_offset":117080,"ancestor_record_offset":116832,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":117176,"parent_record_offset":117080,"ancestor_record_offset":116832,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":117116,"parent_record_offset":117080,"ancestor_record_offset":116832,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":117200,"parent_record_offset":117176,"ancestor_record_offset":116832,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":119168,"parent_record_offset":119144,"ancestor_record_offset":116832,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":120588,"parent_record_offset":120564,"ancestor_record_offset":116832,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":120612,"parent_record_offset":120588,"ancestor_record_offset":116832,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::parse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1800,"record_offset":121200,"parent_record_offset":121176,"ancestor_record_offset":116832,"inlinee":6807,"source_line":2765,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":120540,"parent_record_offset":120516,"ancestor_record_offset":116832,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":117376,"parent_record_offset":117352,"ancestor_record_offset":116832,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":117400,"parent_record_offset":117376,"ancestor_record_offset":116832,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1800,"record_offset":120416,"parent_record_offset":120392,"ancestor_record_offset":116832,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::HistorySnapshotId::from_file_stem
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1800,"record_offset":121176,"parent_record_offset":120008,"ancestor_record_offset":116832,"inlinee":35954,"source_line":290,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1800,"record_offset":122348,"parent_record_offset":122324,"ancestor_record_offset":116832,"inlinee":82160,"source_line":273,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":122036,"parent_record_offset":122008,"ancestor_record_offset":116832,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":122068,"parent_record_offset":122036,"ancestor_record_offset":116832,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":122008,"parent_record_offset":121756,"ancestor_record_offset":116832,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":121868,"parent_record_offset":121756,"ancestor_record_offset":116832,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":122420,"parent_record_offset":121756,"ancestor_record_offset":116832,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":121952,"parent_record_offset":121756,"ancestor_record_offset":116832,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1800,"record_offset":121812,"parent_record_offset":121756,"ancestor_record_offset":116832,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1800,"record_offset":122300,"parent_record_offset":122276,"ancestor_record_offset":116832,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":121648,"parent_record_offset":121624,"ancestor_record_offset":116832,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":122276,"parent_record_offset":122168,"ancestor_record_offset":116832,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":121680,"parent_record_offset":121648,"ancestor_record_offset":116832,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":122216,"parent_record_offset":122192,"ancestor_record_offset":116832,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":122512,"parent_record_offset":121756,"ancestor_record_offset":116832,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":122192,"parent_record_offset":122168,"ancestor_record_offset":116832,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":121732,"parent_record_offset":121708,"ancestor_record_offset":116832,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":121708,"parent_record_offset":121648,"ancestor_record_offset":116832,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":124860,"parent_record_offset":124828,"ancestor_record_offset":116832,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":124772,"parent_record_offset":124748,"ancestor_record_offset":116832,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":121756,"parent_record_offset":121732,"ancestor_record_offset":116832,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":124828,"parent_record_offset":124772,"ancestor_record_offset":116832,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":124800,"parent_record_offset":124772,"ancestor_record_offset":116832,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":121840,"parent_record_offset":121756,"ancestor_record_offset":116832,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":124892,"parent_record_offset":124860,"ancestor_record_offset":116832,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":124748,"parent_record_offset":124724,"ancestor_record_offset":116832,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":122168,"parent_record_offset":121756,"ancestor_record_offset":116832,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1800,"record_offset":121624,"parent_record_offset":121600,"ancestor_record_offset":116832,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1800,"record_offset":119084,"parent_record_offset":119060,"ancestor_record_offset":116832,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":117540,"parent_record_offset":117516,"ancestor_record_offset":116832,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":122708,"parent_record_offset":120008,"ancestor_record_offset":116832,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":117516,"parent_record_offset":117492,"ancestor_record_offset":116832,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":117564,"parent_record_offset":117540,"ancestor_record_offset":116832,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":122568,"parent_record_offset":120008,"ancestor_record_offset":116832,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":121600,"parent_record_offset":120008,"ancestor_record_offset":116832,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1800,"record_offset":120368,"parent_record_offset":120344,"ancestor_record_offset":116832,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":117260,"parent_record_offset":117236,"ancestor_record_offset":116832,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":117320,"parent_record_offset":117236,"ancestor_record_offset":116832,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":122820,"parent_record_offset":120008,"ancestor_record_offset":116832,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::BufReader::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\buffered\bufreader.rs
//     evidence={"module":1800,"record_offset":117640,"parent_record_offset":117604,"ancestor_record_offset":116832,"inlinee":15860,"source_line":102,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":1800,"record_offset":117676,"parent_record_offset":117640,"ancestor_record_offset":116832,"inlinee":15753,"source_line":34,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":1800,"record_offset":118048,"parent_record_offset":118024,"ancestor_record_offset":116832,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1800,"record_offset":117288,"parent_record_offset":117260,"ancestor_record_offset":116832,"inlinee":7537,"source_line":215,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1800,"record_offset":120392,"parent_record_offset":120368,"ancestor_record_offset":116832,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::nuke_unreferenced_shapshots
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 871
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15679
// 首条位置: {"module":1800,"inlinee":87673}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1800,"record_offset":120112,"parent_record_offset":120008,"ancestor_record_offset":120008,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":120168,"parent_record_offset":120008,"ancestor_record_offset":120008,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":120460,"parent_record_offset":120008,"ancestor_record_offset":120008,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":122652,"parent_record_offset":120008,"ancestor_record_offset":120008,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":120316,"parent_record_offset":120008,"ancestor_record_offset":120008,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":120664,"parent_record_offset":120008,"ancestor_record_offset":120008,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::HistorySnapshotId::from_file_stem
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1800,"record_offset":121176,"parent_record_offset":120008,"ancestor_record_offset":120008,"inlinee":35954,"source_line":290,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":122708,"parent_record_offset":120008,"ancestor_record_offset":120008,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":122568,"parent_record_offset":120008,"ancestor_record_offset":120008,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1800,"record_offset":121600,"parent_record_offset":120008,"ancestor_record_offset":120008,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1800,"record_offset":122820,"parent_record_offset":120008,"ancestor_record_offset":120008,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":120912,"parent_record_offset":120888,"ancestor_record_offset":120008,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":120936,"parent_record_offset":120912,"ancestor_record_offset":120008,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":120960,"parent_record_offset":120936,"ancestor_record_offset":120008,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":120888,"parent_record_offset":120832,"ancestor_record_offset":120008,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":120860,"parent_record_offset":120832,"ancestor_record_offset":120008,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":120832,"parent_record_offset":120808,"ancestor_record_offset":120008,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":120808,"parent_record_offset":120784,"ancestor_record_offset":120008,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":123868,"parent_record_offset":123844,"ancestor_record_offset":120008,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1800,"record_offset":124352,"parent_record_offset":124328,"ancestor_record_offset":120008,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::to_digit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":1800,"record_offset":121372,"parent_record_offset":121272,"ancestor_record_offset":120008,"inlinee":6771,"source_line":402,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1800,"record_offset":120516,"parent_record_offset":120488,"ancestor_record_offset":120008,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1800,"record_offset":121892,"parent_record_offset":121868,"ancestor_record_offset":120008,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1800,"record_offset":121976,"parent_record_offset":121952,"ancestor_record_offset":120008,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1800,"record_offset":122392,"parent_record_offset":121756,"ancestor_record_offset":120008,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1800,"record_offset":121488,"parent_record_offset":121432,"ancestor_record_offset":120008,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::can_not_overflow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1800,"record_offset":121344,"parent_record_offset":121272,"ancestor_record_offset":120008,"inlinee":6770,"source_line":1521,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":124920,"parent_record_offset":124892,"ancestor_record_offset":120008,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":124480,"parent_record_offset":124452,"ancestor_record_offset":120008,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$40::from_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1800,"record_offset":121224,"parent_record_offset":121200,"ancestor_record_offset":120008,"inlinee":6859,"source_line":1578,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$41::from_ascii_radix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1800,"record_offset":121272,"parent_record_offset":121248,"ancestor_record_offset":120008,"inlinee":6862,"source_line":1708,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$41::from_str_radix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1800,"record_offset":121248,"parent_record_offset":121224,"ancestor_record_offset":120008,"inlinee":6861,"source_line":1629,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":121400,"parent_record_offset":121372,"ancestor_record_offset":120008,"inlinee":4145,"source_line":2493,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":121432,"parent_record_offset":121272,"ancestor_record_offset":120008,"inlinee":6867,"source_line":1160,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":121460,"parent_record_offset":121432,"ancestor_record_offset":120008,"inlinee":6868,"source_line":3050,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1800,"record_offset":122124,"parent_record_offset":122096,"ancestor_record_offset":120008,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1800,"record_offset":122096,"parent_record_offset":122068,"ancestor_record_offset":120008,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1800,"record_offset":120344,"parent_record_offset":120316,"ancestor_record_offset":120008,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":122676,"parent_record_offset":122652,"ancestor_record_offset":120008,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1800,"record_offset":120488,"parent_record_offset":120460,"ancestor_record_offset":120008,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":121916,"parent_record_offset":121892,"ancestor_record_offset":120008,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":120688,"parent_record_offset":120664,"ancestor_record_offset":120008,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":120136,"parent_record_offset":120112,"ancestor_record_offset":120008,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1800,"record_offset":122240,"parent_record_offset":122216,"ancestor_record_offset":120008,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":120228,"parent_record_offset":120168,"ancestor_record_offset":120008,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":120256,"parent_record_offset":120168,"ancestor_record_offset":120008,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1800,"record_offset":120200,"parent_record_offset":120168,"ancestor_record_offset":120008,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":120280,"parent_record_offset":120256,"ancestor_record_offset":120008,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":120588,"parent_record_offset":120564,"ancestor_record_offset":120008,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1800,"record_offset":120612,"parent_record_offset":120588,"ancestor_record_offset":120008,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::parse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1800,"record_offset":121200,"parent_record_offset":121176,"ancestor_record_offset":120008,"inlinee":6807,"source_line":2765,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1800,"record_offset":120540,"parent_record_offset":120516,"ancestor_record_offset":120008,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":122592,"parent_record_offset":122568,"ancestor_record_offset":120008,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1800,"record_offset":122616,"parent_record_offset":122592,"ancestor_record_offset":120008,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1800,"record_offset":120416,"parent_record_offset":120392,"ancestor_record_offset":120008,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1800,"record_offset":122348,"parent_record_offset":122324,"ancestor_record_offset":120008,"inlinee":82160,"source_line":273,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":122036,"parent_record_offset":122008,"ancestor_record_offset":120008,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":122068,"parent_record_offset":122036,"ancestor_record_offset":120008,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1800,"record_offset":122008,"parent_record_offset":121756,"ancestor_record_offset":120008,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":121868,"parent_record_offset":121756,"ancestor_record_offset":120008,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":122420,"parent_record_offset":121756,"ancestor_record_offset":120008,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1800,"record_offset":121952,"parent_record_offset":121756,"ancestor_record_offset":120008,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1800,"record_offset":121812,"parent_record_offset":121756,"ancestor_record_offset":120008,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1800,"record_offset":122300,"parent_record_offset":122276,"ancestor_record_offset":120008,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":121648,"parent_record_offset":121624,"ancestor_record_offset":120008,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":122276,"parent_record_offset":122168,"ancestor_record_offset":120008,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1800,"record_offset":121680,"parent_record_offset":121648,"ancestor_record_offset":120008,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":122216,"parent_record_offset":122192,"ancestor_record_offset":120008,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":122512,"parent_record_offset":121756,"ancestor_record_offset":120008,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":122192,"parent_record_offset":122168,"ancestor_record_offset":120008,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":121732,"parent_record_offset":121708,"ancestor_record_offset":120008,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":121708,"parent_record_offset":121648,"ancestor_record_offset":120008,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":124860,"parent_record_offset":124828,"ancestor_record_offset":120008,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":124772,"parent_record_offset":124748,"ancestor_record_offset":120008,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":121756,"parent_record_offset":121732,"ancestor_record_offset":120008,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":124828,"parent_record_offset":124772,"ancestor_record_offset":120008,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":124800,"parent_record_offset":124772,"ancestor_record_offset":120008,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":121840,"parent_record_offset":121756,"ancestor_record_offset":120008,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":124892,"parent_record_offset":124860,"ancestor_record_offset":120008,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":124748,"parent_record_offset":124724,"ancestor_record_offset":120008,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1800,"record_offset":122168,"parent_record_offset":121756,"ancestor_record_offset":120008,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1800,"record_offset":121624,"parent_record_offset":121600,"ancestor_record_offset":120008,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":122756,"parent_record_offset":122732,"ancestor_record_offset":120008,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1800,"record_offset":122732,"parent_record_offset":122708,"ancestor_record_offset":120008,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1800,"record_offset":122780,"parent_record_offset":122756,"ancestor_record_offset":120008,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1800,"record_offset":120368,"parent_record_offset":120344,"ancestor_record_offset":120008,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1800,"record_offset":120392,"parent_record_offset":120368,"ancestor_record_offset":120008,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01800.jsonl.gz"}
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
// 完整结构及原始类型记录: save_thread.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::save_thread::AutoSaveError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::save::save_thread::AutoSaveError>", "constructor": "enum2$", "arguments": ["country_core::systems::save::save_thread::AutoSaveError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >", "constructor": "tuple$", "arguments": ["std::path::PathBuf", "enum2$<country_core::systems::save::save_thread::AutoSaveError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$12::clone<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >,alloc::alloc::Global>", "constructor": "alloc::vec::impl$12::clone", "arguments": ["tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::cmp::impl$4::equal_same_length<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >,tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> > >", "constructor": "core::slice::cmp::impl$4::equal_same_length", "arguments": ["tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >", "tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::save_thread::AutosaveErrorState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 37
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<country_core::systems::save::save_thread::AutosaveErrorState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<country_core::systems::save::save_thread::AutosaveErrorState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpsc::Receiver", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$20::drop::closure_env$2<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::impl$20::drop::closure_env$2", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<country_core::systems::save::save_thread::AutosaveErrorState>,std::sync::mpmc::impl$20::drop::closure_env$2<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<country_core::systems::save::save_thread::AutosaveErrorState>", "std::sync::mpmc::impl$20::drop::closure_env$2<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Sender<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpsc::Sender", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Sender<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Sender<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::SendError<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpsc::SendError", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::SendError<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::SendError<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<country_core::systems::save::save_thread::AutosaveErrorState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<country_core::systems::save::save_thread::AutosaveErrorState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::try_recv<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::array::Channel::try_recv", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::try_recv<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::Channel::try_recv", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::error::SendTimeoutError<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::error::SendTimeoutError", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<std::sync::mpmc::error::SendTimeoutError<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "enum2$", "arguments": ["std::sync::mpmc::error::SendTimeoutError<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<country_core::systems::save::save_thread::AutosaveErrorState> > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<std::sync::mpmc::error::SendTimeoutError<country_core::systems::save::save_thread::AutosaveErrorState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<country_core::systems::save::save_thread::AutosaveErrorState> > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<country_core::systems::save::save_thread::AutosaveErrorState> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::context::impl$0::with::closure$2<std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState>,enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<country_core::systems::save::save_thread::AutosaveErrorState> > > > >", "constructor": "std::sync::mpmc::context::impl$0::with::closure$2", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState>", "enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<country_core::systems::save::save_thread::AutosaveErrorState> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState> > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::try_recv<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::list::Channel::try_recv", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<country_core::systems::save::save_thread::AutosaveErrorState>,std::sync::mpmc::impl$6::drop::closure_env$2<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<country_core::systems::save::save_thread::AutosaveErrorState>", "std::sync::mpmc::impl$6::drop::closure_env$2<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::disconnect<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::Channel::disconnect", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::save_thread::CurrentSaveFile
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::save::save_thread::CurrentSaveFile>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::save::save_thread::CurrentSaveFile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::save_thread::DeferredSection
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::systems::save::save_thread::DeferredSection>", "constructor": "core::option::Option", "arguments": ["country_core::systems::save::save_thread::DeferredSection"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::systems::save::save_thread::DeferredSection> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::systems::save::save_thread::DeferredSection>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<enum2$<core::option::Option<country_core::systems::save::save_thread::DeferredSection> >,13>", "constructor": "array$", "arguments": ["enum2$<core::option::Option<country_core::systems::save::save_thread::DeferredSection> >", "13"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<array$<enum2$<core::option::Option<country_core::systems::save::save_thread::DeferredSection> >,13> >", "constructor": "core::ptr::drop_in_place", "arguments": ["array$<enum2$<core::option::Option<country_core::systems::save::save_thread::DeferredSection> >,13>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::save_thread::SnapshotGcCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 20
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>", "constructor": "enum2$", "arguments": ["country_core::systems::save::save_thread::SnapshotGcCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Sender<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpsc::Sender", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Sender<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Sender<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::SendError<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpsc::SendError", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::SendError<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::SendError<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::send::closure$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::array::impl$1::send::closure$0", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::try_recv<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::array::Channel::try_recv", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::try_recv<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::zero::Channel::try_recv", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::error::SendTimeoutError<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::error::SendTimeoutError", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >", "constructor": "enum2$", "arguments": ["std::sync::mpmc::error::SendTimeoutError<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::context::impl$0::with::closure$2<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >,enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > > > >", "constructor": "std::sync::mpmc::context::impl$0::with::closure$2", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::try_recv<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::list::Channel::try_recv", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::impl$20::drop::closure_env$2", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >,std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::save_thread::UnsavedFileList
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<country_core::systems::save::save_thread::UnsavedFileList,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["country_core::systems::save::save_thread::UnsavedFileList", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: save_thread.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: save_thread.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 27; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<alloc::string::String,std::path::PathBuf> >,ref$<tuple$<alloc::string::String,std::path::PathBuf> >)
// 候选参数类型: ["ref$<tuple$<alloc::string::String,std::path::PathBuf> >", "ref$<tuple$<alloc::string::String,std::path::PathBuf> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::clutter_registry::ClutterId>,ref$<country_core::resources::clutter_registry::ClutterId>)
// 候选参数类型: ["ref$<country_core::resources::clutter_registry::ClutterId>", "ref$<country_core::resources::clutter_registry::ClutterId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<str$> >,ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >", "ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::AutosaveErrorState::is_ok
// 信息 ID: 3b537c8b72e8e52c228e3e49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$12::render @ 0x7214fc -> 0xcb3030 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::save_error::ui @ 0x23c8e0d -> 0xcb3030 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1800, inline_record=3568; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::CurrentSaveFile::backup_save
// 信息 ID: b8b897880376319ede21a1bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=46000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::CurrentSaveFile::change_section
// 信息 ID: 716adb3d64479b8b92a32d9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::save_thread @ 0x1b6e96d -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b6fe53 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b6fea3 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b6fef3 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b6ff43 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b6ff93 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b6ffe3 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b70033 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b70083 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b700d3 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b70123 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b70173 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b701c3 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b70213 -> 0x1b6f1d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync
// 信息 ID: bcb4c3264d6d93bc1766a73e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::save_thread @ 0x1b6ea03 -> 0x1b6fc60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::execute_save_cmd @ 0x1b72792 -> 0x1b6fc60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::CurrentSaveFile::perform_manual_backup
// 信息 ID: a15e004ff0952bd7e1143fac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=70468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::CurrentSaveFile::replace_edits
// 信息 ID: 431752a99312e52eadb9d506
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=57976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::UnsavedFileList::add
// 信息 ID: bf392a9da41b169b4a7c52e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::execute_save_cmd @ 0x1b730ab -> 0x1b708c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::UnsavedFileList::contains_file
// 信息 ID: 8ca777852d8e31823001d550
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::load_completion_checker::is_session_ok_to_load @ 0xb3213c -> 0x1b706e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::save_thread @ 0x1b6e894 -> 0x1b706e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::capture_snapshot
// 信息 ID: d3c62bc074025d6294d6d33e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=77864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::execute_open_file_cmd
// 信息 ID: 131ac13ef8b32f44500bfae2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=84156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::execute_open_file_cmd::closure$0
// 信息 ID: 3e937d58404900c127d07464
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_thread::execute_open_file_cmd", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=88044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::execute_open_file_cmd::closure$2
// 信息 ID: 0fec8565bd27bc64e16f7080
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::save::save_thread::execute_open_file_cmd", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2 <- core::ptr::drop_in_place<std::thread::lifecycle::spawn_unchecked::MaybeDangling<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2 <- std::sys::backtrace::__rust_begin_short_backtrace<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2 <- std::thread::lifecycle::spawn_unchecked<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2 <- core::ops::function::FnOnce::call_once<std::thread::lifecycle::spawn_unchecked::closure_env$1<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2 <- core::ptr::drop_in_place<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2 <- core::ptr::drop_in_place<std::thread::lifecycle::spawn_unchecked::closure_env$1<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2,tuple$<> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2,tuple$<> >; module=535, inline_record=111288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::execute_save_cmd
// 信息 ID: ccae7f5ab664df59b7e0c204
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::save_thread @ 0x1b6e079 -> 0x1b70aa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::full_write_history_file
// 信息 ID: ec49c51211921601a73ffce1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=92672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$0::backup_save::closure$1
// 信息 ID: 245cc11b35f2ada22540848a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::save::save_thread::impl$0::backup_save", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1 <- core::iter::adapters::filter::impl$3::next<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::save::save_backup_list::BackupSaveFile,core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0>,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 2 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$0::backup_save::closure$2
// 信息 ID: 6f58ac9856923e606d50e7e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::save::save_thread::impl$0::backup_save", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2 <- core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2 <- core::slice::sort::stable::driftsort_main<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>,alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2 <- core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2 <- core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; module=528, inline_record=42204; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; module=528, inline_record=42372; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; module=3393, inline_record=8380; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; module=3393, inline_record=8548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$0::perform_manual_backup::closure$0
// 信息 ID: edbc97fd2e6943a305bf791e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_thread::impl$0::perform_manual_backup", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::perform_manual_backup::closure_env$0 <- anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,country_core::systems::save::save_thread::impl$0::perform_manual_backup::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,country_core::systems::save::save_thread::impl$0::perform_manual_backup::closure_env$0>; module=1253, inline_record=32032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$10::clone
// 信息 ID: 43b6577d609db0099f84ca52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1800, inline_record=4048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$11::default
// 信息 ID: 3379afe068634893de2c8f48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1800, inline_record=2632; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1800, inline_record=2832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$13::eq
// 信息 ID: 210a84a456100f073f582407
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1800, inline_record=3468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$1::contains_file::closure$0
// 信息 ID: 141932dd12159197037d5bd2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_thread::impl$1::contains_file", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::UnsavedFileList::contains_file; module=1800, inline_record=34560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$2::from
// 信息 ID: 26362bf3c767c3f6255f5478
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::save_thread::closure$0 @ 0x15f056d -> 0x1b6de50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::save_thread @ 0x1b6e11e -> 0x1b6de50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$5::clone
// 信息 ID: edda9350209ba8358a749c50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >,alloc::alloc::Global>; module=1643, inline_record=257728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$8::eq
// 信息 ID: 72c82f81b164a0c9ce81456b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal_same_length<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >,tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> > >; module=1662, inline_record=35008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::nuke_unreferenced_shapshots
// 信息 ID: f64573731e3d85dd4f659b17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::snapshot_gc_thread; module=1800, inline_record=120008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::save_thread
// 信息 ID: 8b55818abdd16b1c61fab4a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0,tuple$<> > @ 0xa41fef -> 0x1b6df50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::save_thread::closure$0
// 信息 ID: 48d1cc34166deab028504078
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_thread::save_thread", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_thread::save_thread::closure_env$0 <- alloc::vec::Vec::retain_mut<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global,country_core::systems::save::save_thread::save_thread::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] alloc::vec::Vec::retain_mut<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global,country_core::systems::save::save_thread::save_thread::closure_env$0> @ 0x15f1896 -> 0x15f0480 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::Vec::retain_mut<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global,country_core::systems::save::save_thread::save_thread::closure_env$0> @ 0x15f1903 -> 0x15f0480 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::save_thread::closure$1
// 信息 ID: 7f421d2779524c524191e07c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::save::save_thread::save_thread", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_thread::save_thread::closure_env$1 <- hashbrown::map::HashMap::retain<std::path::PathBuf,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,hashbrown::set::impl$1::retain::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,country_core::systems::save::save_thread::save_thread::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] hashbrown::map::HashMap::retain<std::path::PathBuf,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,hashbrown::set::impl$1::retain::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,country_core::systems::save::save_thread::save_thread::closure_env$1> >; module=1658, inline_record=396004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::save_thread::closure$2
// 信息 ID: 011742321c09e379dbab52a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::save::save_thread::save_thread", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1800, inline_record=9944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::snapshot_gc_thread
// 信息 ID: fd2976d2bfdc41f2bf863894
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2,tuple$<> > @ 0xa41e47 -> 0x1b79fa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
