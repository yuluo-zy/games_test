// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/device.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: cb3f7a419a780e031edf98adde7d84bae5661a47843a528101776d30c3b8883d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: device.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 245, 690, 702, 703, 705, 707, 1189, 1657, 1935, 1936, 1965, 1978, 2310
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use bumpalo; // namespace candidate
//   package identities: [{"name":"bumpalo","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.18.1"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use gpu_allocator; // namespace candidate
//   package identities: [{"name":"gpu-allocator","source":"local_or_path","version":"0.26.0"}]
// use gpu_profiler; // namespace candidate
//   package identities: [{"name":"gpu-profiler","source":"local_or_path","version":"0.1.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use leekspin; // namespace candidate
//   package identities: [{"name":"leekspin","source":"local_or_path","version":"0.1.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":43500,"rva":12526944,"va":5381236064}
// 编译实例 RVAs: 0xbf2560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..convert..From$LT$alloc..borrow..Cow$LT$str$GT$$GT$$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1610865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e871941800","size":5,"soft_fail":false,"target_va":5382848208,"va":5381237338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$gpu_allocator..AllocatorDebugSettings$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t13450492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8fc3ccd00","size":5,"soft_fail":false,"target_va":5394695312,"va":5381244815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1406272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e840751500","size":5,"soft_fail":false,"target_va":5382644528,"va":5381238251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a360
//     owner=unresolved_namespace; count=3
//     target_aliases=27 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11189058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8be4455ff","size":5,"soft_fail":false,"target_va":5370061664,"va":5381250717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7612374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e82ad88bff","size":5,"soft_fail":false,"target_va":5373629488,"va":5381241857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6557423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e811f19bff","size":5,"soft_fail":false,"target_va":5374679664,"va":5381237082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e6620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6357239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e809ff9eff","size":5,"soft_fail":false,"target_va":5374895648,"va":5381252882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8f70
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6288122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8060da0ff","size":5,"soft_fail":false,"target_va":5374971760,"va":5381259877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6155003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e80515a2ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381236486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6155952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e85011a2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381237451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=8
//     target_aliases=69 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5722016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e860b0a8ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381238043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d090
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-613089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e81fa5f6ff","size":5,"soft_fail":false,"target_va":5380624528,"va":5381237612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdcfe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-106021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8db61feff","size":5,"soft_fail":false,"target_va":5381148640,"va":5381254656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdd070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-105327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e89164feff","size":5,"soft_fail":false,"target_va":5381148784,"va":5381254106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdd100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-105923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e83d62feff","size":5,"soft_fail":false,"target_va":5381148928,"va":5381254846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bddce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-101382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8fa73feff","size":5,"soft_fail":false,"target_va":5381151968,"va":5381253345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bde330
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-97209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e84784feff","size":5,"soft_fail":false,"target_va":5381153584,"va":5381250788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bde530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-99754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8567afeff","size":5,"soft_fail":false,"target_va":5381154096,"va":5381253845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bde570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-100472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e88877feff","size":5,"soft_fail":false,"target_va":5381154160,"va":5381254627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdecc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-99242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8567cfeff","size":5,"soft_fail":false,"target_va":5381156032,"va":5381255269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdedf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-97002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e81685feff","size":5,"soft_fail":false,"target_va":5381156336,"va":5381253333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdf360
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-93046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e88a94feff","size":5,"soft_fail":false,"target_va":5381157728,"va":5381250769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be0b70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-86860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8b4acfeff","size":5,"soft_fail":false,"target_va":5381163888,"va":5381250743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be0d50
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-86328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8c8aefeff","size":5,"soft_fail":false,"target_va":5381164368,"va":5381250691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-80228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e89cc6feff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381250575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bee200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e80c85ffff","size":5,"soft_fail":false,"target_va":5381218816,"va":5381250287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b7510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13374667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8cb14cc00","size":5,"soft_fail":false,"target_va":5394625808,"va":5381251136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b8610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13383128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8d835cc00","size":5,"soft_fail":false,"target_va":5394630160,"va":5381247027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418c2970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13422843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8fbd0cc00","size":5,"soft_fail":false,"target_va":5394671984,"va":5381249136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30049482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8ca84ca01","size":5,"soft_fail":false,"target_va":5411285568,"va":5381236081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30609936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e81012d301","size":5,"soft_fail":false,"target_va":5411847152,"va":5381237211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=24
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30607433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e84908d301","size":5,"soft_fail":false,"target_va":5411847168,"va":5381239730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30597703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e847e2d201","size":5,"soft_fail":false,"target_va":5411847200,"va":5381249492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142924010
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30611190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8f616d301","size":5,"soft_fail":false,"target_va":5411848208,"va":5381237013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-33602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8be7cffff","size":5,"soft_fail":false,"target_va":5381217616,"va":5381251213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30290135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8d730ce01","size":5,"soft_fail":false,"target_va":5411540992,"va":5381250852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::ffi::c_str::_$LT$impl$u20$core..ffi..c_str..CStr$GT$::to_string_lossy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1603139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e843761800","size":5,"soft_fail":false,"target_va":5382840176,"va":5381237032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1603829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8f5781800","size":5,"soft_fail":false,"target_va":5382840208,"va":5381236374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6880470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e82a0397ff","size":5,"soft_fail":false,"target_va":5374356912,"va":5381237377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc::new<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::allocator::GpuAllocator> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-34115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8bd7affff","size":5,"soft_fail":false,"target_va":5381217424,"va":5381251534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::_$LT$impl$u20$core..convert..From$LT$alloc..vec..Vec$LT$T$C$A$GT$$GT$$u20$for$u20$alloc..boxed..Box$LT$$u5b$T$u5d$$C$A$GT$$GT$::from
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13448070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e88633cd00","size":5,"soft_fail":false,"target_va":5394699552,"va":5381251477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::extensions_generated::ext::debug_utils::Device::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13447316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e89430cd00","size":5,"soft_fail":false,"target_va":5394696608,"va":5381249287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::extensions_generated::ext::full_screen_exclusive::Device::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13448616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8a835cd00","size":5,"soft_fail":false,"target_va":5394697824,"va":5381249203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::extensions_generated::khr::swapchain::Device::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13448901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8c536cd00","size":5,"soft_fail":false,"target_va":5394698336,"va":5381249430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::instance::Instance::create_device
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e89c9dffff","size":5,"soft_fail":false,"target_va":5381216240,"va":5381241423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30295845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e82547ce01","size":5,"soft_fail":false,"target_va":5411534720,"va":5381238870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1405951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8ff731500","size":5,"soft_fail":false,"target_va":5382644656,"va":5381238700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30282381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e88d12ce01","size":5,"soft_fail":false,"target_va":5411535312,"va":5381252926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30289073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8b12cce01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381252986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_div_by_zero
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30290505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e84932ce01","size":5,"soft_fail":false,"target_va":5411543520,"va":5381253010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30289763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8632fce01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381253112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::AllocationSizes::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13450487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8f73ccd00","size":5,"soft_fail":false,"target_va":5394695328,"va":5381244836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_profiler::backend::ash::Profiler::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-profiler","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t110252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8acae0100","size":5,"soft_fail":false,"target_va":5381361440,"va":5381251183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t101346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8e28b0100","size":5,"soft_fail":false,"target_va":5381347712,"va":5381246361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBufferPool::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-36306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e82e72ffff","size":5,"soft_fail":false,"target_va":5381215424,"va":5381251725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::query_device_extension_support::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t26611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e8f3670000","size":5,"soft_fail":false,"target_va":5381263280,"va":5381236664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::resources::ManagedResources::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13413513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e889accc00","size":5,"soft_fail":false,"target_va":5394663648,"va":5381250130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_bytes
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t51465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e809c90000","size":5,"soft_fail":false,"target_va":5381293360,"va":5381241890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::semaphore::create_raw_timeline_semaphore
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13449219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e80338cd00","size":5,"soft_fail":false,"target_va":5394695952,"va":5381246728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13421652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381236064,"raw_hex":"e854cccc00","size":5,"soft_fail":false,"target_va":5394673296,"va":5381251639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*176(%r15)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381236064,"raw_hex":"41ff97b0000000","size":7,"soft_fail":false,"va":5381239899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":48888,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":60448,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$2::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":57908,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4594,"source_line":153,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":49620,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":60484,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":50548,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_pipeline_cache
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":59960,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39702,"source_line":2236,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::get_device_queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":57780,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39676,"source_line":2357,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::impl$5::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":54364,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39667,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_support
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":703,"record_offset":57812,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39678,"source_line":12,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_properties2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":703,"record_offset":59752,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39697,"source_line":161,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::impl$4::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":703,"record_offset":53880,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39659,"source_line":13,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DeviceQueueCreateInfo::queue_family_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":49068,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39639,"source_line":1274,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DeviceQueueCreateInfo::queue_priorities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":49592,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39641,"source_line":1279,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceProperties2::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":59548,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39696,"source_line":11108,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$268::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":59932,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39701,"source_line":5167,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$2825::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":59196,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39688,"source_line":30594,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$35::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":49040,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39637,"source_line":1251,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$652::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":59224,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39690,"source_line":11084,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":703,"record_offset":49096,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39640,"source_line":1386,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":703,"record_offset":49704,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39642,"source_line":1681,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":703,"record_offset":59140,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":53364,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":48152,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":703,"record_offset":60684,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":703,"record_offset":52572,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":703,"record_offset":53564,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":703,"record_offset":53708,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":7173,"source_line":624,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":703,"record_offset":47808,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":58048,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::next_multiple_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":53420,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39657,"source_line":3233,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":59836,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":47620,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":57284,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":61416,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":43972,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":44000,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU16::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":59780,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39699,"source_line":2956,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":57604,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":10634,"source_line":154,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":60712,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":10633,"source_line":422,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$11::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":703,"record_offset":61076,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":12821,"source_line":1151,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":703,"record_offset":48184,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":48068,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":703,"record_offset":59868,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":38999,"source_line":119,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::create_samplers
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/sampler.rs
//     evidence={"module":703,"record_offset":58132,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39681,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::validate_physical_device_requirements
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":43832,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39628,"source_line":736,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::create_raw_device
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":50740,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39644,"source_line":795,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::query_device_extension_support
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":44028,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39630,"source_line":1041,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":703,"record_offset":60936,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39706,"source_line":15,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":703,"record_offset":57964,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39680,"source_line":54,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::stats::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/stats.rs
//     evidence={"module":703,"record_offset":61132,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":39708,"source_line":3,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":44232,"parent_record_offset":44208,"ancestor_record_offset":43500,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":44256,"parent_record_offset":44232,"ancestor_record_offset":43500,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":45196,"parent_record_offset":45168,"ancestor_record_offset":43500,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":44176,"parent_record_offset":44028,"ancestor_record_offset":43500,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":44208,"parent_record_offset":44176,"ancestor_record_offset":43500,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":45168,"parent_record_offset":45104,"ancestor_record_offset":43500,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":61872,"parent_record_offset":61848,"ancestor_record_offset":43500,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":57932,"parent_record_offset":57908,"ancestor_record_offset":43500,"inlinee":4596,"source_line":558,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$2::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":57988,"parent_record_offset":57964,"ancestor_record_offset":43500,"inlinee":4594,"source_line":153,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":703,"record_offset":43872,"parent_record_offset":43832,"ancestor_record_offset":43500,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":703,"record_offset":43928,"parent_record_offset":43900,"ancestor_record_offset":43500,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":44544,"parent_record_offset":44520,"ancestor_record_offset":43500,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":45136,"parent_record_offset":45104,"ancestor_record_offset":43500,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":45104,"parent_record_offset":45076,"ancestor_record_offset":43500,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":44592,"parent_record_offset":44568,"ancestor_record_offset":43500,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":44568,"parent_record_offset":44544,"ancestor_record_offset":43500,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":45076,"parent_record_offset":45052,"ancestor_record_offset":43500,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":44352,"parent_record_offset":44328,"ancestor_record_offset":43500,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":44328,"parent_record_offset":44028,"ancestor_record_offset":43500,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":47680,"parent_record_offset":47644,"ancestor_record_offset":43500,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":47644,"parent_record_offset":47620,"ancestor_record_offset":43500,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44856,"parent_record_offset":44804,"ancestor_record_offset":43500,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44520,"parent_record_offset":44496,"ancestor_record_offset":43500,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44496,"parent_record_offset":44464,"ancestor_record_offset":43500,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44376,"parent_record_offset":44352,"ancestor_record_offset":43500,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":46792,"parent_record_offset":44028,"ancestor_record_offset":43500,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44412,"parent_record_offset":44028,"ancestor_record_offset":43500,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44804,"parent_record_offset":44028,"ancestor_record_offset":43500,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":58072,"parent_record_offset":58048,"ancestor_record_offset":43500,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":58996,"parent_record_offset":58132,"ancestor_record_offset":43500,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44440,"parent_record_offset":44028,"ancestor_record_offset":43500,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44464,"parent_record_offset":44440,"ancestor_record_offset":43500,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::instance::Instance::get_physical_device_features2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":703,"record_offset":51408,"parent_record_offset":50740,"ancestor_record_offset":43500,"inlinee":39654,"source_line":151,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$32::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":703,"record_offset":53912,"parent_record_offset":53880,"ancestor_record_offset":43500,"inlinee":39661,"source_line":112,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$33::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":703,"record_offset":54408,"parent_record_offset":54364,"ancestor_record_offset":43500,"inlinee":39669,"source_line":395,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$35::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":703,"record_offset":54136,"parent_record_offset":53880,"ancestor_record_offset":43500,"inlinee":39663,"source_line":2713,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$36::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":703,"record_offset":56236,"parent_record_offset":54364,"ancestor_record_offset":43500,"inlinee":39671,"source_line":2966,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$39::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":703,"record_offset":56488,"parent_record_offset":54364,"ancestor_record_offset":43500,"inlinee":39673,"source_line":3307,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$41::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":703,"record_offset":54304,"parent_record_offset":53880,"ancestor_record_offset":43500,"inlinee":39665,"source_line":3574,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$42::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":703,"record_offset":56712,"parent_record_offset":54364,"ancestor_record_offset":43500,"inlinee":39675,"source_line":3610,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::QueueFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":703,"record_offset":47992,"parent_record_offset":47968,"ancestor_record_offset":43500,"inlinee":39634,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::DeviceCreateInfo::push_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":51932,"parent_record_offset":50740,"ancestor_record_offset":43500,"inlinee":39656,"source_line":1383,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::PhysicalDeviceFeatures2::push_next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":51000,"parent_record_offset":50740,"ancestor_record_offset":43500,"inlinee":39652,"source_line":11056,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$15::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":59252,"parent_record_offset":59224,"ancestor_record_offset":43500,"inlinee":39692,"source_line":889,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$2798::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":50944,"parent_record_offset":50740,"ancestor_record_offset":43500,"inlinee":39648,"source_line":30394,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$287::default
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":58404,"parent_record_offset":58380,"ancestor_record_offset":43500,"inlinee":39685,"source_line":5362,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$2906::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":50916,"parent_record_offset":50740,"ancestor_record_offset":43500,"inlinee":39646,"source_line":31432,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$345::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":59396,"parent_record_offset":59252,"ancestor_record_offset":43500,"inlinee":39694,"source_line":6825,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$646::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":50972,"parent_record_offset":50740,"ancestor_record_offset":43500,"inlinee":39650,"source_line":11031,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":59992,"parent_record_offset":59960,"ancestor_record_offset":43500,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":57848,"parent_record_offset":57812,"ancestor_record_offset":43500,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":57872,"parent_record_offset":57848,"ancestor_record_offset":43500,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":703,"record_offset":51148,"parent_record_offset":51124,"ancestor_record_offset":43500,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":703,"record_offset":49132,"parent_record_offset":49096,"ancestor_record_offset":43500,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":703,"record_offset":49736,"parent_record_offset":49704,"ancestor_record_offset":43500,"inlinee":39643,"source_line":1552,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":703,"record_offset":49160,"parent_record_offset":49132,"ancestor_record_offset":43500,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":703,"record_offset":49200,"parent_record_offset":49160,"ancestor_record_offset":43500,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::ChunkFooter::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":703,"record_offset":52972,"parent_record_offset":52936,"ancestor_record_offset":43500,"inlinee":12170,"source_line":375,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::dealloc_chunk_list
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":703,"record_offset":52936,"parent_record_offset":52912,"ancestor_record_offset":43500,"inlinee":12168,"source_line":395,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$6::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":703,"record_offset":52912,"parent_record_offset":52888,"ancestor_record_offset":43500,"inlinee":16119,"source_line":387,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":703,"record_offset":49524,"parent_record_offset":49160,"ancestor_record_offset":43500,"inlinee":15494,"source_line":1865,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":703,"record_offset":49296,"parent_record_offset":49200,"ancestor_record_offset":43500,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":703,"record_offset":49788,"parent_record_offset":49736,"ancestor_record_offset":43500,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":703,"record_offset":61904,"parent_record_offset":61872,"ancestor_record_offset":43500,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":703,"record_offset":49812,"parent_record_offset":49788,"ancestor_record_offset":43500,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":49240,"parent_record_offset":49200,"ancestor_record_offset":43500,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":49404,"parent_record_offset":49380,"ancestor_record_offset":43500,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":49380,"parent_record_offset":49200,"ancestor_record_offset":43500,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=106
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":703,"record_offset":53936,"parent_record_offset":53912,"ancestor_record_offset":43500,"inlinee":4549,"source_line":165,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":50348,"parent_record_offset":50308,"ancestor_record_offset":43500,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":46180,"parent_record_offset":46152,"ancestor_record_offset":43500,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":703,"record_offset":46536,"parent_record_offset":44028,"ancestor_record_offset":43500,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":703,"record_offset":45680,"parent_record_offset":45656,"ancestor_record_offset":43500,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":703,"record_offset":45764,"parent_record_offset":45740,"ancestor_record_offset":43500,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":703,"record_offset":45256,"parent_record_offset":44028,"ancestor_record_offset":43500,"inlinee":12585,"source_line":264,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":703,"record_offset":45284,"parent_record_offset":45256,"ancestor_record_offset":43500,"inlinee":12587,"source_line":721,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":45312,"parent_record_offset":45284,"ancestor_record_offset":43500,"inlinee":12589,"source_line":3874,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":45704,"parent_record_offset":45680,"ancestor_record_offset":43500,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":46356,"parent_record_offset":45544,"ancestor_record_offset":43500,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":59356,"parent_record_offset":59332,"ancestor_record_offset":43500,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":703,"record_offset":47832,"parent_record_offset":47808,"ancestor_record_offset":43500,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":703,"record_offset":59076,"parent_record_offset":59052,"ancestor_record_offset":43500,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":703,"record_offset":51052,"parent_record_offset":51028,"ancestor_record_offset":43500,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":703,"record_offset":51076,"parent_record_offset":51052,"ancestor_record_offset":43500,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":703,"record_offset":51124,"parent_record_offset":51100,"ancestor_record_offset":43500,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":703,"record_offset":50380,"parent_record_offset":50308,"ancestor_record_offset":43500,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":703,"record_offset":50308,"parent_record_offset":50272,"ancestor_record_offset":43500,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":703,"record_offset":50272,"parent_record_offset":49736,"ancestor_record_offset":43500,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":703,"record_offset":58096,"parent_record_offset":58072,"ancestor_record_offset":43500,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":59108,"parent_record_offset":58132,"ancestor_record_offset":43500,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":51028,"parent_record_offset":51000,"ancestor_record_offset":43500,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":59052,"parent_record_offset":58132,"ancestor_record_offset":43500,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":51100,"parent_record_offset":51076,"ancestor_record_offset":43500,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":703,"record_offset":59308,"parent_record_offset":59284,"ancestor_record_offset":43500,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":703,"record_offset":49428,"parent_record_offset":49404,"ancestor_record_offset":43500,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":703,"record_offset":61932,"parent_record_offset":61904,"ancestor_record_offset":43500,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":703,"record_offset":59284,"parent_record_offset":59252,"ancestor_record_offset":43500,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":50408,"parent_record_offset":50380,"ancestor_record_offset":43500,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":46952,"parent_record_offset":46920,"ancestor_record_offset":43500,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":703,"record_offset":45912,"parent_record_offset":45884,"ancestor_record_offset":43500,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":703,"record_offset":45884,"parent_record_offset":45856,"ancestor_record_offset":43500,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":703,"record_offset":47944,"parent_record_offset":47856,"ancestor_record_offset":43500,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":48036,"parent_record_offset":47808,"ancestor_record_offset":43500,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":43900,"parent_record_offset":43872,"ancestor_record_offset":43500,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":49496,"parent_record_offset":49160,"ancestor_record_offset":43500,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":703,"record_offset":48240,"parent_record_offset":48208,"ancestor_record_offset":43500,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":703,"record_offset":48268,"parent_record_offset":48208,"ancestor_record_offset":43500,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":45004,"parent_record_offset":44804,"ancestor_record_offset":43500,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":53000,"parent_record_offset":52972,"ancestor_record_offset":43500,"inlinee":7359,"source_line":2074,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":44660,"parent_record_offset":44636,"ancestor_record_offset":43500,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":46032,"parent_record_offset":46008,"ancestor_record_offset":43500,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":49344,"parent_record_offset":49320,"ancestor_record_offset":43500,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":49320,"parent_record_offset":49296,"ancestor_record_offset":43500,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":59332,"parent_record_offset":59308,"ancestor_record_offset":43500,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":47712,"parent_record_offset":47680,"ancestor_record_offset":43500,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":44760,"parent_record_offset":44700,"ancestor_record_offset":43500,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":44284,"parent_record_offset":44256,"ancestor_record_offset":43500,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":44976,"parent_record_offset":44804,"ancestor_record_offset":43500,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":52340,"parent_record_offset":50740,"ancestor_record_offset":43500,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":52384,"parent_record_offset":50740,"ancestor_record_offset":43500,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":46264,"parent_record_offset":46236,"ancestor_record_offset":43500,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":46292,"parent_record_offset":46264,"ancestor_record_offset":43500,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":703,"record_offset":44636,"parent_record_offset":44464,"ancestor_record_offset":43500,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":47856,"parent_record_offset":47832,"ancestor_record_offset":43500,"inlinee":4466,"source_line":296,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":44700,"parent_record_offset":44028,"ancestor_record_offset":43500,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":46208,"parent_record_offset":46180,"ancestor_record_offset":43500,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":60508,"parent_record_offset":60484,"ancestor_record_offset":43500,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":48092,"parent_record_offset":48068,"ancestor_record_offset":43500,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":59804,"parent_record_offset":59780,"ancestor_record_offset":43500,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":48116,"parent_record_offset":48092,"ancestor_record_offset":43500,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":47740,"parent_record_offset":47680,"ancestor_record_offset":43500,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":47772,"parent_record_offset":47644,"ancestor_record_offset":43500,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":703,"record_offset":45824,"parent_record_offset":45796,"ancestor_record_offset":43500,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":703,"record_offset":45856,"parent_record_offset":45824,"ancestor_record_offset":43500,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":703,"record_offset":45796,"parent_record_offset":45544,"ancestor_record_offset":43500,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":703,"record_offset":45656,"parent_record_offset":45544,"ancestor_record_offset":43500,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":703,"record_offset":46384,"parent_record_offset":45544,"ancestor_record_offset":43500,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":703,"record_offset":45740,"parent_record_offset":45544,"ancestor_record_offset":43500,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":703,"record_offset":45628,"parent_record_offset":45544,"ancestor_record_offset":43500,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":703,"record_offset":46152,"parent_record_offset":46124,"ancestor_record_offset":43500,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":45408,"parent_record_offset":45380,"ancestor_record_offset":43500,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":45436,"parent_record_offset":45408,"ancestor_record_offset":43500,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":46124,"parent_record_offset":45956,"ancestor_record_offset":43500,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":45468,"parent_record_offset":45436,"ancestor_record_offset":43500,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":46064,"parent_record_offset":45980,"ancestor_record_offset":43500,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":46008,"parent_record_offset":45980,"ancestor_record_offset":43500,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":46476,"parent_record_offset":45544,"ancestor_record_offset":43500,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":45980,"parent_record_offset":45956,"ancestor_record_offset":43500,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":45520,"parent_record_offset":45496,"ancestor_record_offset":43500,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":45496,"parent_record_offset":45436,"ancestor_record_offset":43500,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":45544,"parent_record_offset":45520,"ancestor_record_offset":43500,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":45600,"parent_record_offset":45544,"ancestor_record_offset":43500,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":45956,"parent_record_offset":45544,"ancestor_record_offset":43500,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":703,"record_offset":45380,"parent_record_offset":45352,"ancestor_record_offset":43500,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":60848,"parent_record_offset":60712,"ancestor_record_offset":43500,"inlinee":10634,"source_line":154,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":703,"record_offset":61100,"parent_record_offset":61076,"ancestor_record_offset":43500,"inlinee":12822,"source_line":371,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$11::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":703,"record_offset":60960,"parent_record_offset":60936,"ancestor_record_offset":43500,"inlinee":12821,"source_line":1151,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":48296,"parent_record_offset":48208,"ancestor_record_offset":43500,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":703,"record_offset":48208,"parent_record_offset":48184,"ancestor_record_offset":43500,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":48320,"parent_record_offset":48296,"ancestor_record_offset":43500,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::crash::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/crash.rs
//     evidence={"module":703,"record_offset":60736,"parent_record_offset":60712,"ancestor_record_offset":43500,"inlinee":39704,"source_line":5,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::impl$4::create::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":47968,"parent_record_offset":47944,"ancestor_record_offset":43500,"inlinee":39632,"source_line":297,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::impl$4::create::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":48520,"parent_record_offset":48492,"ancestor_record_offset":43500,"inlinee":39635,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::impl$4::create::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":59164,"parent_record_offset":59140,"ancestor_record_offset":43500,"inlinee":39686,"source_line":477,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::sampler::impl$0::create_samplers::closure$0
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/sampler.rs
//     evidence={"module":703,"record_offset":58380,"parent_record_offset":58132,"ancestor_record_offset":43500,"inlinee":39683,"source_line":25,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":703,"record_offset":60760,"parent_record_offset":60736,"ancestor_record_offset":43500,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":703,"record_offset":45352,"parent_record_offset":44028,"ancestor_record_offset":43500,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":703,"record_offset":60808,"parent_record_offset":60784,"ancestor_record_offset":43500,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":703,"record_offset":60784,"parent_record_offset":60760,"ancestor_record_offset":43500,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::query_device_extension_support::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":71624,"rva":12554160,"va":5381263280}
// 编译实例 RVAs: 0xbf8fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1380609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381263280,"raw_hex":"e801111500","size":5,"soft_fail":false,"target_va":5382644528,"va":5381263914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6583751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381263280,"raw_hex":"e8398a9bff","size":5,"soft_fail":false,"target_va":5374679664,"va":5381263410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5747697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381263280,"raw_hex":"e80f4ca8ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381263724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30583611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381263280,"raw_hex":"e83babd201","size":5,"soft_fail":false,"target_va":5411847152,"va":5381263536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6908396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381263280,"raw_hex":"e8149696ff","size":5,"soft_fail":false,"target_va":5374355568,"va":5381263959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ffi::c_str::CStr::to_str
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1605896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381263280,"raw_hex":"e808811800","size":5,"soft_fail":false,"target_va":5382869248,"va":5381263347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30278775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381263280,"raw_hex":"e87704ce01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381264100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":73328,"parent_record_offset":71624,"ancestor_record_offset":71624,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":73084,"parent_record_offset":71624,"ancestor_record_offset":71624,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":71792,"parent_record_offset":71624,"ancestor_record_offset":71624,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":703,"record_offset":73152,"parent_record_offset":71624,"ancestor_record_offset":71624,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":73000,"parent_record_offset":71624,"ancestor_record_offset":71624,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":703,"record_offset":71828,"parent_record_offset":71624,"ancestor_record_offset":71624,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":73388,"parent_record_offset":73364,"ancestor_record_offset":71624,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":73436,"parent_record_offset":73412,"ancestor_record_offset":71624,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":73412,"parent_record_offset":73388,"ancestor_record_offset":71624,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":73364,"parent_record_offset":73328,"ancestor_record_offset":71624,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":72644,"parent_record_offset":72616,"ancestor_record_offset":71624,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":703,"record_offset":72144,"parent_record_offset":72120,"ancestor_record_offset":71624,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":703,"record_offset":72228,"parent_record_offset":72204,"ancestor_record_offset":71624,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":72168,"parent_record_offset":72144,"ancestor_record_offset":71624,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":72820,"parent_record_offset":72008,"ancestor_record_offset":71624,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":703,"record_offset":72376,"parent_record_offset":72348,"ancestor_record_offset":71624,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":703,"record_offset":72348,"parent_record_offset":72320,"ancestor_record_offset":71624,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":703,"record_offset":73208,"parent_record_offset":73176,"ancestor_record_offset":71624,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":703,"record_offset":73236,"parent_record_offset":73176,"ancestor_record_offset":71624,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":72496,"parent_record_offset":72472,"ancestor_record_offset":71624,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":73476,"parent_record_offset":73328,"ancestor_record_offset":71624,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":72728,"parent_record_offset":72700,"ancestor_record_offset":71624,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":72756,"parent_record_offset":72728,"ancestor_record_offset":71624,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":72672,"parent_record_offset":72644,"ancestor_record_offset":71624,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":73024,"parent_record_offset":73000,"ancestor_record_offset":71624,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":73048,"parent_record_offset":73024,"ancestor_record_offset":71624,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":703,"record_offset":72288,"parent_record_offset":72260,"ancestor_record_offset":71624,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":703,"record_offset":72320,"parent_record_offset":72288,"ancestor_record_offset":71624,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":703,"record_offset":72260,"parent_record_offset":72008,"ancestor_record_offset":71624,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":703,"record_offset":72120,"parent_record_offset":72008,"ancestor_record_offset":71624,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":703,"record_offset":72848,"parent_record_offset":72008,"ancestor_record_offset":71624,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":703,"record_offset":72204,"parent_record_offset":72008,"ancestor_record_offset":71624,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":703,"record_offset":72092,"parent_record_offset":72008,"ancestor_record_offset":71624,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":703,"record_offset":72616,"parent_record_offset":72588,"ancestor_record_offset":71624,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":71876,"parent_record_offset":71852,"ancestor_record_offset":71624,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":71900,"parent_record_offset":71876,"ancestor_record_offset":71624,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":72588,"parent_record_offset":72420,"ancestor_record_offset":71624,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":71932,"parent_record_offset":71900,"ancestor_record_offset":71624,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":72528,"parent_record_offset":72444,"ancestor_record_offset":71624,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":72472,"parent_record_offset":72444,"ancestor_record_offset":71624,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":72940,"parent_record_offset":72008,"ancestor_record_offset":71624,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":72444,"parent_record_offset":72420,"ancestor_record_offset":71624,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":71984,"parent_record_offset":71960,"ancestor_record_offset":71624,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":71960,"parent_record_offset":71900,"ancestor_record_offset":71624,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":72008,"parent_record_offset":71984,"ancestor_record_offset":71624,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":72064,"parent_record_offset":72008,"ancestor_record_offset":71624,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":72420,"parent_record_offset":72008,"ancestor_record_offset":71624,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":703,"record_offset":71852,"parent_record_offset":71828,"ancestor_record_offset":71624,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":73264,"parent_record_offset":73176,"ancestor_record_offset":71624,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":703,"record_offset":73176,"parent_record_offset":73152,"ancestor_record_offset":71624,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":73288,"parent_record_offset":73264,"ancestor_record_offset":71624,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::Queue::with_raw
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C16
// 首条位置: {"module":707,"inlinee":39958}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":707,"record_offset":30444,"parent_record_offset":29560,"ancestor_record_offset":29560,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":29964,"parent_record_offset":29560,"ancestor_record_offset":29560,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":707,"record_offset":29628,"parent_record_offset":29560,"ancestor_record_offset":29560,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::impl$2::queue_submit::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":707,"record_offset":30120,"parent_record_offset":29560,"ancestor_record_offset":29560,"inlinee":39960,"source_line":342,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":30248,"parent_record_offset":30224,"ancestor_record_offset":29560,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":30296,"parent_record_offset":30272,"ancestor_record_offset":29560,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":30272,"parent_record_offset":30248,"ancestor_record_offset":29560,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":30224,"parent_record_offset":30200,"ancestor_record_offset":29560,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":30200,"parent_record_offset":30176,"ancestor_record_offset":29560,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$30::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":30152,"parent_record_offset":30120,"ancestor_record_offset":29560,"inlinee":4954,"source_line":3825,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":707,"record_offset":30176,"parent_record_offset":30152,"ancestor_record_offset":29560,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::queue_submit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":707,"record_offset":30348,"parent_record_offset":30120,"ancestor_record_offset":29560,"inlinee":39961,"source_line":2464,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":707,"record_offset":30380,"parent_record_offset":30348,"ancestor_record_offset":29560,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":707,"record_offset":30404,"parent_record_offset":30380,"ancestor_record_offset":29560,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":707,"record_offset":29780,"parent_record_offset":29688,"ancestor_record_offset":29560,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":30472,"parent_record_offset":30444,"ancestor_record_offset":29560,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":30592,"parent_record_offset":30528,"ancestor_record_offset":29560,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":30028,"parent_record_offset":29992,"ancestor_record_offset":29560,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":29808,"parent_record_offset":29688,"ancestor_record_offset":29560,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":30876,"parent_record_offset":30748,"ancestor_record_offset":29560,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":29724,"parent_record_offset":29688,"ancestor_record_offset":29560,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":30616,"parent_record_offset":30592,"ancestor_record_offset":29560,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":29832,"parent_record_offset":29808,"ancestor_record_offset":29560,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":29748,"parent_record_offset":29724,"ancestor_record_offset":29560,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":30900,"parent_record_offset":30876,"ancestor_record_offset":29560,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":30500,"parent_record_offset":30472,"ancestor_record_offset":29560,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":707,"record_offset":30720,"parent_record_offset":30692,"ancestor_record_offset":29560,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":29992,"parent_record_offset":29964,"ancestor_record_offset":29560,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":30528,"parent_record_offset":30500,"ancestor_record_offset":29560,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":29868,"parent_record_offset":29652,"ancestor_record_offset":29560,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":30788,"parent_record_offset":30748,"ancestor_record_offset":29560,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":29688,"parent_record_offset":29652,"ancestor_record_offset":29560,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":29652,"parent_record_offset":29628,"ancestor_record_offset":29560,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":30748,"parent_record_offset":30720,"ancestor_record_offset":29560,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":29896,"parent_record_offset":29868,"ancestor_record_offset":29560,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":30564,"parent_record_offset":30528,"ancestor_record_offset":29560,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::Queue::with_raw<rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0,enum2$<core::result::Result<bool,ash::vk::enums::Result> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":707,"record_offset":9616,"rva":12598640,"va":5381307760}
// 编译实例 RVAs: 0xc03d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140041280
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12332305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381307760,"raw_hex":"e8efd243ff","size":5,"soft_fail":false,"target_va":5368976000,"va":5381308300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11150088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381307760,"raw_hex":"e8f8dc55ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381308211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13358416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381307760,"raw_hex":"e850d5cb00","size":5,"soft_fail":false,"target_va":5394666368,"va":5381307947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30184245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381307760,"raw_hex":"e83593cc01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381308118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30186653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381307760,"raw_hex":"e89d9ccc01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381308142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30181109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381307760,"raw_hex":"e8f586cc01","size":5,"soft_fail":false,"target_va":5411488944,"va":5381307830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30183495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381307760,"raw_hex":"e84790cc01","size":5,"soft_fail":false,"target_va":5411491664,"va":5381308164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t515744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381307760,"raw_hex":"e8a0de0700","size":5,"soft_fail":false,"target_va":5381823600,"va":5381307851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t516010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381307760,"raw_hex":"e8aadf0700","size":5,"soft_fail":false,"target_va":5381824016,"va":5381308001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1744(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381307760,"raw_hex":"ff97d0060000","size":6,"soft_fail":false,"va":5381307958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":707,"record_offset":10476,"parent_record_offset":9616,"ancestor_record_offset":9616,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":10216,"parent_record_offset":9616,"ancestor_record_offset":9616,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":707,"record_offset":9884,"parent_record_offset":9616,"ancestor_record_offset":9616,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::impl$2::present_image::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":707,"record_offset":10372,"parent_record_offset":9616,"ancestor_record_offset":9616,"inlinee":39944,"source_line":581,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::swapchain::Device::queue_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":707,"record_offset":10436,"parent_record_offset":10372,"ancestor_record_offset":9616,"inlinee":39945,"source_line":81,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":707,"record_offset":10036,"parent_record_offset":9944,"ancestor_record_offset":9616,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":10504,"parent_record_offset":10476,"ancestor_record_offset":9616,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":10624,"parent_record_offset":10560,"ancestor_record_offset":9616,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":10280,"parent_record_offset":10244,"ancestor_record_offset":9616,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":10064,"parent_record_offset":9944,"ancestor_record_offset":9616,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":10908,"parent_record_offset":10780,"ancestor_record_offset":9616,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":9980,"parent_record_offset":9944,"ancestor_record_offset":9616,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":10648,"parent_record_offset":10624,"ancestor_record_offset":9616,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":10088,"parent_record_offset":10064,"ancestor_record_offset":9616,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":10004,"parent_record_offset":9980,"ancestor_record_offset":9616,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":10932,"parent_record_offset":10908,"ancestor_record_offset":9616,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":10532,"parent_record_offset":10504,"ancestor_record_offset":9616,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":707,"record_offset":10752,"parent_record_offset":10724,"ancestor_record_offset":9616,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":10244,"parent_record_offset":10216,"ancestor_record_offset":9616,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":10560,"parent_record_offset":10532,"ancestor_record_offset":9616,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":10124,"parent_record_offset":9908,"ancestor_record_offset":9616,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":10820,"parent_record_offset":10780,"ancestor_record_offset":9616,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":9944,"parent_record_offset":9908,"ancestor_record_offset":9616,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":9908,"parent_record_offset":9884,"ancestor_record_offset":9616,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":707,"record_offset":10780,"parent_record_offset":10752,"ancestor_record_offset":9616,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":10148,"parent_record_offset":10124,"ancestor_record_offset":9616,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":10596,"parent_record_offset":10560,"ancestor_record_offset":9616,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::queue_present
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":707,"record_offset":10404,"parent_record_offset":10372,"ancestor_record_offset":9616,"inlinee":39360,"source_line":32,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$6::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3DB6
// 首条位置: {"module":245,"inlinee":15798}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::Queue::update_pending_submissions
// 记录形式: compiled_procedure; instances=1
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":27388,"rva":12520336,"va":5381229456}
// 编译实例 RVAs: 0xbf0b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30313136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229456,"raw_hex":"e8b08ace01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381229739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13436785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229456,"raw_hex":"e87107cd00","size":5,"soft_fail":false,"target_va":5394666368,"va":5381229578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1144(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381229456,"raw_hex":"ff9078040000","size":6,"soft_fail":false,"va":5381229487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":27732,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":4909,"source_line":1608,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":27904,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":4905,"source_line":1698,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::get_semaphore_counter_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":27552,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":39621,"source_line":679,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":28052,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":11109,"source_line":1425,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":28148,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::read_submit_timeline_semaphore_compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":27668,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":39329,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::read_submit_timeline_semaphore_universal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":27700,"parent_record_offset":27388,"ancestor_record_offset":27388,"inlinee":39328,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":27764,"parent_record_offset":27732,"ancestor_record_offset":27388,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":27800,"parent_record_offset":27764,"ancestor_record_offset":27388,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":27828,"parent_record_offset":27800,"ancestor_record_offset":27388,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":27856,"parent_record_offset":27828,"ancestor_record_offset":27388,"inlinee":13999,"source_line":2832,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":27580,"parent_record_offset":27552,"ancestor_record_offset":27388,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":27604,"parent_record_offset":27580,"ancestor_record_offset":27388,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":27628,"parent_record_offset":27604,"ancestor_record_offset":27388,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":28108,"parent_record_offset":28080,"ancestor_record_offset":27388,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$17::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":703,"record_offset":28080,"parent_record_offset":28052,"ancestor_record_offset":27388,"inlinee":39623,"source_line":82,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::Queue::reclaim_command_buffer_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":28180,"rva":12520640,"va":5381229760}
// 编译实例 RVAs: 0xbf0cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=3
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11076568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e828fc56ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381234691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405df600
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6366089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e877dc9eff","size":5,"soft_fail":false,"target_va":5374866944,"va":5381233028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6149899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8f528a2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381231398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c6b0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-610264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e828b0f6ff","size":5,"soft_fail":false,"target_va":5380622000,"va":5381232259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bddf00
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-82861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e853bcfeff","size":5,"soft_fail":false,"target_va":5381152512,"va":5381235368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdef60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-75660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e874d8feff","size":5,"soft_fail":false,"target_va":5381156704,"va":5381232359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418c7ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13460233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e80963cd00","size":5,"soft_fail":false,"target_va":5394692832,"va":5381232594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30615809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e80129d301","size":5,"soft_fail":false,"target_va":5411847168,"va":5381231354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6875552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8601697ff","size":5,"soft_fail":false,"target_va":5374357296,"va":5381232843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30249601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e88192cd01","size":5,"soft_fail":false,"target_va":5411481920,"va":5381232314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30307970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e88276ce01","size":5,"soft_fail":false,"target_va":5411541584,"va":5381233609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30308009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8a976ce01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381233762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30248164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8e48ccd01","size":5,"soft_fail":false,"target_va":5411481740,"va":5381233571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30308572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8dc78ce01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381233487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30308505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e89978ce01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381233786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-13532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e824cbffff","size":5,"soft_fail":false,"target_va":5381216320,"va":5381229847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30313438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8de8bce01","size":5,"soft_fail":false,"target_va":5411543360,"va":5381229917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30260088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e878bbcd01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381232275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30262462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8bec4cd01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381232333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t592305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8b1090900","size":5,"soft_fail":false,"target_va":5381823600,"va":5381231290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t592591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8cf0a0900","size":5,"soft_fail":false,"target_va":5381824016,"va":5381231420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t155161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8195e0200","size":5,"soft_fail":false,"target_va":5381385536,"va":5381230370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t160350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e85e720200","size":5,"soft_fail":false,"target_va":5381390640,"va":5381230285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t154913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8215d0200","size":5,"soft_fail":false,"target_va":5381386480,"va":5381231562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t154888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8085d0200","size":5,"soft_fail":false,"target_va":5381384704,"va":5381229811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t30506662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8a67ed101","size":5,"soft_fail":false,"target_va":5411737808,"va":5381231141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t13438886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8a60fcd00","size":5,"soft_fail":false,"target_va":5394673152,"va":5381234261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30303667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381229760,"raw_hex":"e8b365ce01","size":5,"soft_fail":false,"target_va":5411537104,"va":5381233432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":30812,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":4909,"source_line":1608,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":31228,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":4905,"source_line":1698,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":38284,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":31576,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":38068,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":33752,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":703,"record_offset":38124,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":31144,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":11109,"source_line":1425,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":703,"record_offset":31460,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":31420,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":34764,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":33596,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":29828,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":28396,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":28852,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBufferPool::recycle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":703,"record_offset":40908,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":39581,"source_line":278,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::DescriptorPoolDiscardPile::reclaim_descriptor_pools
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":703,"record_offset":38312,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":39625,"source_line":33,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":703,"record_offset":40664,"parent_record_offset":28180,"ancestor_record_offset":28180,"inlinee":39627,"source_line":76,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":34380,"parent_record_offset":34356,"ancestor_record_offset":28180,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":34356,"parent_record_offset":34268,"ancestor_record_offset":28180,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":31352,"parent_record_offset":31228,"ancestor_record_offset":28180,"inlinee":4907,"source_line":181,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":30924,"parent_record_offset":30896,"ancestor_record_offset":28180,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":30836,"parent_record_offset":30812,"ancestor_record_offset":28180,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":31024,"parent_record_offset":30836,"ancestor_record_offset":28180,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":30868,"parent_record_offset":30836,"ancestor_record_offset":28180,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":703,"record_offset":30896,"parent_record_offset":30868,"ancestor_record_offset":28180,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":30956,"parent_record_offset":30924,"ancestor_record_offset":28180,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":31048,"parent_record_offset":31024,"ancestor_record_offset":28180,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":33904,"parent_record_offset":33876,"ancestor_record_offset":28180,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":30980,"parent_record_offset":30956,"ancestor_record_offset":28180,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":34296,"parent_record_offset":34268,"ancestor_record_offset":28180,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":34268,"parent_record_offset":34244,"ancestor_record_offset":28180,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":33972,"parent_record_offset":33936,"ancestor_record_offset":28180,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":31096,"parent_record_offset":31072,"ancestor_record_offset":28180,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":31072,"parent_record_offset":31048,"ancestor_record_offset":28180,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":33936,"parent_record_offset":33904,"ancestor_record_offset":28180,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":34244,"parent_record_offset":34220,"ancestor_record_offset":28180,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":39440,"parent_record_offset":39404,"ancestor_record_offset":28180,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":39404,"parent_record_offset":39380,"ancestor_record_offset":28180,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":33832,"parent_record_offset":33788,"ancestor_record_offset":28180,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":38748,"parent_record_offset":38564,"ancestor_record_offset":28180,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":38564,"parent_record_offset":38532,"ancestor_record_offset":28180,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":34092,"parent_record_offset":33832,"ancestor_record_offset":28180,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":41080,"parent_record_offset":40908,"ancestor_record_offset":28180,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":33876,"parent_record_offset":33832,"ancestor_record_offset":28180,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":40948,"parent_record_offset":40908,"ancestor_record_offset":28180,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":38500,"parent_record_offset":38312,"ancestor_record_offset":28180,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":38936,"parent_record_offset":38912,"ancestor_record_offset":28180,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":703,"record_offset":34148,"parent_record_offset":34124,"ancestor_record_offset":28180,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":703,"record_offset":34196,"parent_record_offset":34172,"ancestor_record_offset":28180,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":703,"record_offset":38992,"parent_record_offset":38936,"ancestor_record_offset":28180,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":703,"record_offset":39148,"parent_record_offset":38564,"ancestor_record_offset":28180,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":703,"record_offset":39100,"parent_record_offset":39076,"ancestor_record_offset":28180,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":703,"record_offset":38532,"parent_record_offset":38500,"ancestor_record_offset":28180,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":703,"record_offset":33788,"parent_record_offset":33752,"ancestor_record_offset":28180,"inlinee":8579,"source_line":35,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":30164,"parent_record_offset":30136,"ancestor_record_offset":28180,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":30216,"parent_record_offset":30192,"ancestor_record_offset":28180,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":30192,"parent_record_offset":30164,"ancestor_record_offset":28180,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":30112,"parent_record_offset":30084,"ancestor_record_offset":28180,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":30136,"parent_record_offset":30112,"ancestor_record_offset":28180,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":30308,"parent_record_offset":30136,"ancestor_record_offset":28180,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":30420,"parent_record_offset":30396,"ancestor_record_offset":28180,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":40716,"parent_record_offset":40692,"ancestor_record_offset":28180,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":31192,"parent_record_offset":31168,"ancestor_record_offset":28180,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":40692,"parent_record_offset":40664,"ancestor_record_offset":28180,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":38884,"parent_record_offset":38860,"ancestor_record_offset":28180,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":38860,"parent_record_offset":38564,"ancestor_record_offset":28180,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":38912,"parent_record_offset":38884,"ancestor_record_offset":28180,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":703,"record_offset":30240,"parent_record_offset":30216,"ancestor_record_offset":28180,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":34324,"parent_record_offset":34296,"ancestor_record_offset":28180,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":31308,"parent_record_offset":31284,"ancestor_record_offset":28180,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":703,"record_offset":28552,"parent_record_offset":28520,"ancestor_record_offset":28180,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":703,"record_offset":29980,"parent_record_offset":29936,"ancestor_record_offset":28180,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":28800,"parent_record_offset":28772,"ancestor_record_offset":28180,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":30768,"parent_record_offset":29936,"ancestor_record_offset":28180,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":30372,"parent_record_offset":30084,"ancestor_record_offset":28180,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":31376,"parent_record_offset":31352,"ancestor_record_offset":28180,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":38196,"parent_record_offset":38124,"ancestor_record_offset":28180,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":39472,"parent_record_offset":39440,"ancestor_record_offset":28180,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":29264,"parent_record_offset":29240,"ancestor_record_offset":28180,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":38224,"parent_record_offset":38124,"ancestor_record_offset":28180,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":29240,"parent_record_offset":29216,"ancestor_record_offset":28180,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":29448,"parent_record_offset":29420,"ancestor_record_offset":28180,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":31512,"parent_record_offset":31484,"ancestor_record_offset":28180,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":30264,"parent_record_offset":30240,"ancestor_record_offset":28180,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":40988,"parent_record_offset":40948,"ancestor_record_offset":28180,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":41028,"parent_record_offset":40988,"ancestor_record_offset":28180,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":29192,"parent_record_offset":29168,"ancestor_record_offset":28180,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":29216,"parent_record_offset":29192,"ancestor_record_offset":28180,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":29520,"parent_record_offset":29492,"ancestor_record_offset":28180,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":29168,"parent_record_offset":29052,"ancestor_record_offset":28180,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":703,"record_offset":28520,"parent_record_offset":28492,"ancestor_record_offset":28180,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":703,"record_offset":28880,"parent_record_offset":28852,"ancestor_record_offset":28180,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":703,"record_offset":29020,"parent_record_offset":28988,"ancestor_record_offset":28180,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":703,"record_offset":28988,"parent_record_offset":28952,"ancestor_record_offset":28180,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":28928,"parent_record_offset":28904,"ancestor_record_offset":28180,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":28952,"parent_record_offset":28928,"ancestor_record_offset":28180,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":28904,"parent_record_offset":28880,"ancestor_record_offset":28180,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":29492,"parent_record_offset":28952,"ancestor_record_offset":28180,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":28492,"parent_record_offset":28460,"ancestor_record_offset":28180,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":28460,"parent_record_offset":28432,"ancestor_record_offset":28180,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":28652,"parent_record_offset":28620,"ancestor_record_offset":28180,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":28748,"parent_record_offset":28652,"ancestor_record_offset":28180,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":28620,"parent_record_offset":28596,"ancestor_record_offset":28180,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":28432,"parent_record_offset":28396,"ancestor_record_offset":28180,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":28772,"parent_record_offset":28748,"ancestor_record_offset":28180,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":29052,"parent_record_offset":29020,"ancestor_record_offset":28180,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":29336,"parent_record_offset":29052,"ancestor_record_offset":28180,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":29308,"parent_record_offset":29052,"ancestor_record_offset":28180,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":29112,"parent_record_offset":29052,"ancestor_record_offset":28180,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":34884,"parent_record_offset":34820,"ancestor_record_offset":28180,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":33660,"parent_record_offset":33624,"ancestor_record_offset":28180,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":34908,"parent_record_offset":34884,"ancestor_record_offset":28180,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":33684,"parent_record_offset":33660,"ancestor_record_offset":28180,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":39500,"parent_record_offset":39440,"ancestor_record_offset":28180,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":39532,"parent_record_offset":39404,"ancestor_record_offset":28180,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":38344,"parent_record_offset":38312,"ancestor_record_offset":28180,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":34792,"parent_record_offset":34764,"ancestor_record_offset":28180,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":33624,"parent_record_offset":33596,"ancestor_record_offset":28180,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":34820,"parent_record_offset":34792,"ancestor_record_offset":28180,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":33716,"parent_record_offset":33624,"ancestor_record_offset":28180,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":34856,"parent_record_offset":34820,"ancestor_record_offset":28180,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":29864,"parent_record_offset":29828,"ancestor_record_offset":28180,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":35032,"parent_record_offset":34992,"ancestor_record_offset":28180,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":35600,"parent_record_offset":35328,"ancestor_record_offset":28180,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":703,"record_offset":30008,"parent_record_offset":29980,"ancestor_record_offset":28180,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":30084,"parent_record_offset":29936,"ancestor_record_offset":28180,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":30344,"parent_record_offset":30084,"ancestor_record_offset":28180,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$17::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":703,"record_offset":31168,"parent_record_offset":31144,"ancestor_record_offset":28180,"inlinee":39623,"source_line":82,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":703,"record_offset":30036,"parent_record_offset":30008,"ancestor_record_offset":28180,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":703,"record_offset":29936,"parent_record_offset":29900,"ancestor_record_offset":28180,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":703,"record_offset":29900,"parent_record_offset":29864,"ancestor_record_offset":28180,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 185
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15437
// 首条位置: {"module":1978,"inlinee":87095}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::leek::bucket_dispose_in_order
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1978,"record_offset":30408,"parent_record_offset":30368,"ancestor_record_offset":30368,"inlinee":39357,"source_line":32,"archive":"_pdb/B/modules/01978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$4::create::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 297
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AD0
// 首条位置: {"module":703,"inlinee":39632}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::QueueFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":703,"record_offset":47992,"parent_record_offset":47968,"ancestor_record_offset":47968,"inlinee":39634,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$4::create::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 315
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AD3
// 首条位置: {"module":703,"inlinee":39635}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$4::create::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 442
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152EB
// 首条位置: {"module":1935,"inlinee":86763}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$2::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1935,"record_offset":21240,"parent_record_offset":20996,"ancestor_record_offset":20996,"inlinee":4594,"source_line":153,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::get_device_queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1935,"record_offset":21112,"parent_record_offset":20996,"ancestor_record_offset":20996,"inlinee":39676,"source_line":2357,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_support
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1935,"record_offset":21144,"parent_record_offset":20996,"ancestor_record_offset":20996,"inlinee":39678,"source_line":12,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1935,"record_offset":21072,"parent_record_offset":20996,"ancestor_record_offset":20996,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1935,"record_offset":21712,"parent_record_offset":20996,"ancestor_record_offset":20996,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::semaphore::create_raw_timeline_semaphore
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/semaphore.rs
//     evidence={"module":1935,"record_offset":21296,"parent_record_offset":20996,"ancestor_record_offset":20996,"inlinee":39746,"source_line":17,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1935,"record_offset":21264,"parent_record_offset":21240,"ancestor_record_offset":20996,"inlinee":4596,"source_line":558,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::create_semaphore
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1935,"record_offset":21564,"parent_record_offset":21296,"ancestor_record_offset":20996,"inlinee":39751,"source_line":2129,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::SemaphoreCreateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1935,"record_offset":21360,"parent_record_offset":21296,"ancestor_record_offset":20996,"inlinee":39750,"source_line":7639,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::SemaphoreTypeCreateInfo::semaphore_type
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1935,"record_offset":21332,"parent_record_offset":21296,"ancestor_record_offset":20996,"inlinee":39748,"source_line":21895,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1935,"record_offset":21592,"parent_record_offset":21564,"ancestor_record_offset":20996,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1935,"record_offset":21180,"parent_record_offset":21144,"ancestor_record_offset":20996,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1935,"record_offset":21204,"parent_record_offset":21180,"ancestor_record_offset":20996,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1935,"record_offset":21508,"parent_record_offset":21484,"ancestor_record_offset":20996,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":1935,"record_offset":21412,"parent_record_offset":21388,"ancestor_record_offset":20996,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1935,"record_offset":21436,"parent_record_offset":21412,"ancestor_record_offset":20996,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1935,"record_offset":21484,"parent_record_offset":21460,"ancestor_record_offset":20996,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1935,"record_offset":21388,"parent_record_offset":21360,"ancestor_record_offset":20996,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1935,"record_offset":21460,"parent_record_offset":21436,"ancestor_record_offset":20996,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1935,"record_offset":21680,"parent_record_offset":21296,"ancestor_record_offset":20996,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$4::create::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B06
// 首条位置: {"module":703,"inlinee":39686}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::device_uuid
// 记录形式: compiled_procedure; instances=1
// 源码行: 607
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":64188,"rva":12551360,"va":5381260480}
// 编译实例 RVAs: 0xbf84c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::physical_device
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":64324,"parent_record_offset":64188,"ancestor_record_offset":64188,"inlinee":39563,"source_line":678,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 621
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":707,"record_offset":14008,"rva":12601408,"va":5381310528}
// 编译实例 RVAs: 0xc04840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11153763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e89dce55ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381311886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405df600
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6444067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e8ddab9dff","size":5,"soft_fail":false,"target_va":5374866944,"va":5381311006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ff290
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6314618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e886a59fff","size":5,"soft_fail":false,"target_va":5374997136,"va":5381311749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6229492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e80cf2a0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381310991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bddf00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-159308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e8b491fdff","size":5,"soft_fail":false,"target_va":5381152512,"va":5381311815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bef4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-87089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e8cfabfeff","size":5,"soft_fail":false,"target_va":5381223584,"va":5381310668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30230357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e85547cd01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381311702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t30230479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e8cf47cd01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381311420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30181038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e8ae86cc01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381311325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30183455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e81f90cc01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381311340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t512994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e8e2d30700","size":5,"soft_fail":false,"target_va":5381823600,"va":5381310601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t512732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e8dcd20700","size":5,"soft_fail":false,"target_va":5381824016,"va":5381311279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::begin
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-95930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381310528,"raw_hex":"e84689feff","size":5,"soft_fail":false,"target_va":5381214768,"va":5381310693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*880(%r10)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381310528,"raw_hex":"41ff9270030000","size":7,"soft_fail":false,"va":5381310878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":14488,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":15236,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":14276,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":14432,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$6::create_buffer::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":707,"record_offset":14516,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":39947,"source_line":330,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":15536,"parent_record_offset":15512,"ancestor_record_offset":14008,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":15512,"parent_record_offset":15420,"ancestor_record_offset":14008,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":15448,"parent_record_offset":15420,"ancestor_record_offset":14008,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":15420,"parent_record_offset":15396,"ancestor_record_offset":14008,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":15396,"parent_record_offset":15372,"ancestor_record_offset":14008,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_copy_buffer
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":707,"record_offset":15204,"parent_record_offset":14516,"ancestor_record_offset":14008,"inlinee":39940,"source_line":1466,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":14456,"parent_record_offset":14432,"ancestor_record_offset":14008,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":707,"record_offset":14816,"parent_record_offset":14724,"ancestor_record_offset":14008,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":707,"record_offset":15480,"parent_record_offset":15448,"ancestor_record_offset":14008,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":15172,"parent_record_offset":15148,"ancestor_record_offset":14008,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":15264,"parent_record_offset":15236,"ancestor_record_offset":14008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":14760,"parent_record_offset":14724,"ancestor_record_offset":14008,"inlinee":39124,"source_line":3344,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":16436,"parent_record_offset":16372,"ancestor_record_offset":14008,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":14340,"parent_record_offset":14304,"ancestor_record_offset":14008,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":14008,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":14364,"parent_record_offset":14340,"ancestor_record_offset":14008,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":14784,"parent_record_offset":14760,"ancestor_record_offset":14008,"inlinee":29644,"source_line":3962,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":16344,"parent_record_offset":16316,"ancestor_record_offset":14008,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":14304,"parent_record_offset":14276,"ancestor_record_offset":14008,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":16372,"parent_record_offset":16344,"ancestor_record_offset":14008,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":14396,"parent_record_offset":14304,"ancestor_record_offset":14008,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":16408,"parent_record_offset":16372,"ancestor_record_offset":14008,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$4::byte_count
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":707,"record_offset":15148,"parent_record_offset":14516,"ancestor_record_offset":14008,"inlinee":39017,"source_line":152,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":707,"record_offset":14724,"parent_record_offset":14636,"ancestor_record_offset":14008,"inlinee":39123,"source_line":54,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$4::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":707,"record_offset":15292,"parent_record_offset":15264,"ancestor_record_offset":14008,"inlinee":39301,"source_line":125,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedRef::get_tracked
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":14572,"parent_record_offset":14516,"ancestor_record_offset":14008,"inlinee":39610,"source_line":327,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":14608,"parent_record_offset":14572,"ancestor_record_offset":14008,"inlinee":8709,"source_line":250,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":14696,"parent_record_offset":14636,"ancestor_record_offset":14008,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$4::track
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":14636,"parent_record_offset":14608,"ancestor_record_offset":14008,"inlinee":39132,"source_line":96,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceMeta::checksum
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":707,"record_offset":14668,"parent_record_offset":14636,"ancestor_record_offset":14008,"inlinee":39121,"source_line":43,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<ref$<slice2$<u8> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 621
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1657,"record_offset":8348,"rva":23736864,"va":5392445984}
// 编译实例 RVAs: 0x16a3220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22288677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e8dbe6abfe","size":5,"soft_fail":false,"target_va":5370158128,"va":5392446800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d4610
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17624827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e80511f3fe","size":5,"soft_fail":false,"target_va":5374821904,"va":5392446726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405df600
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17579473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e82fc2f3fe","size":5,"soft_fail":false,"target_va":5374866944,"va":5392446412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ff290
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17449740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e8f4bcf5fe","size":5,"soft_fail":false,"target_va":5374997136,"va":5392446871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17364898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e85e08f7fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392446397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=4
//     target_aliases=69 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16930147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e89daafdfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5392446174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406868b0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16894590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e88235fefe","size":5,"soft_fail":false,"target_va":5375551664,"va":5392446249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6a70
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11323523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e87d3753ff","size":5,"soft_fail":false,"target_va":5381122672,"va":5392446190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be2370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11276332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e8d4ef53ff","size":5,"soft_fail":false,"target_va":5381170032,"va":5392446359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bef4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11222544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e8f0c154ff","size":5,"soft_fail":false,"target_va":5381223584,"va":5392446123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfc750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11168579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e8bd9455ff","size":5,"soft_fail":false,"target_va":5381277520,"va":5392446094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19045706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e84a9d2201","size":5,"soft_fail":false,"target_va":5411492368,"va":5392446657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19048123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e8bba62201","size":5,"soft_fail":false,"target_va":5411494800,"va":5392446672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10622468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e8fce95dff","size":5,"soft_fail":false,"target_va":5381823600,"va":5392446063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10622599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e879e95dff","size":5,"soft_fail":false,"target_va":5381824016,"va":5392446610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::begin
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11231383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392445984,"raw_hex":"e8699f54ff","size":5,"soft_fail":false,"target_va":5381214768,"va":5392446146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*880(%r12)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392445984,"raw_hex":"41ff942470030000","size":8,"soft_fail":false,"va":5392446288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":8796,"parent_record_offset":8348,"ancestor_record_offset":8348,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":9028,"parent_record_offset":8348,"ancestor_record_offset":8348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1657,"record_offset":8584,"parent_record_offset":8348,"ancestor_record_offset":8348,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1657,"record_offset":8740,"parent_record_offset":8348,"ancestor_record_offset":8348,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$6::create_buffer::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":1657,"record_offset":8824,"parent_record_offset":8348,"ancestor_record_offset":8348,"inlinee":39947,"source_line":330,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":9296,"parent_record_offset":9272,"ancestor_record_offset":8348,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":9272,"parent_record_offset":9180,"ancestor_record_offset":8348,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":9208,"parent_record_offset":9180,"ancestor_record_offset":8348,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":9180,"parent_record_offset":9156,"ancestor_record_offset":8348,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":8348,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_copy_buffer
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":8996,"parent_record_offset":8824,"ancestor_record_offset":8348,"inlinee":39940,"source_line":1466,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":8764,"parent_record_offset":8740,"ancestor_record_offset":8348,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1657,"record_offset":9240,"parent_record_offset":9208,"ancestor_record_offset":8348,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":9056,"parent_record_offset":9028,"ancestor_record_offset":8348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":9672,"parent_record_offset":9608,"ancestor_record_offset":8348,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":8648,"parent_record_offset":8612,"ancestor_record_offset":8348,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":8348,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":8672,"parent_record_offset":8648,"ancestor_record_offset":8348,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1657,"record_offset":9580,"parent_record_offset":9552,"ancestor_record_offset":8348,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1657,"record_offset":8612,"parent_record_offset":8584,"ancestor_record_offset":8348,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1657,"record_offset":9608,"parent_record_offset":9580,"ancestor_record_offset":8348,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1657,"record_offset":8704,"parent_record_offset":8612,"ancestor_record_offset":8348,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1657,"record_offset":9644,"parent_record_offset":9608,"ancestor_record_offset":8348,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedRef::get_tracked
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1657,"record_offset":8868,"parent_record_offset":8824,"ancestor_record_offset":8348,"inlinee":39610,"source_line":327,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1657,"record_offset":8900,"parent_record_offset":8868,"ancestor_record_offset":8348,"inlinee":8709,"source_line":250,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 621
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":707,"record_offset":11992,"rva":12599936,"va":5381309056}
// 编译实例 RVAs: 0xc04280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11151551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e841d755ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381309674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405df600
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6442403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e85db29dff","size":5,"soft_fail":false,"target_va":5374866944,"va":5381309342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ff290
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6312422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e81aae9fff","size":5,"soft_fail":false,"target_va":5374997136,"va":5381309553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6227828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e88cf8a0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381309327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bddf00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-157109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e84b9afdff","size":5,"soft_fail":false,"target_va":5381152512,"va":5381309616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bef4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-85604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e89cb1feff","size":5,"soft_fail":false,"target_va":5381223584,"va":5381309183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30232548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e8e44fcd01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381309511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30182897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e8f18dcc01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381309466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30185314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e86297cc01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381309481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t514469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e8a5d90700","size":5,"soft_fail":false,"target_va":5381823600,"va":5381309126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t514586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e81ada0700","size":5,"soft_fail":false,"target_va":5381824016,"va":5381309425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e8d9f6ffff","size":5,"soft_fail":false,"target_va":5381306880,"va":5381309218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::begin
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-94439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309056,"raw_hex":"e8198ffeff","size":5,"soft_fail":false,"target_va":5381214768,"va":5381309202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":12420,"parent_record_offset":11992,"ancestor_record_offset":11992,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":12448,"parent_record_offset":11992,"ancestor_record_offset":11992,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":12208,"parent_record_offset":11992,"ancestor_record_offset":11992,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":12364,"parent_record_offset":11992,"ancestor_record_offset":11992,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":12748,"parent_record_offset":12724,"ancestor_record_offset":11992,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":12724,"parent_record_offset":12632,"ancestor_record_offset":11992,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":12660,"parent_record_offset":12632,"ancestor_record_offset":11992,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":12632,"parent_record_offset":12608,"ancestor_record_offset":11992,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":12608,"parent_record_offset":12584,"ancestor_record_offset":11992,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":12388,"parent_record_offset":12364,"ancestor_record_offset":11992,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":707,"record_offset":12692,"parent_record_offset":12660,"ancestor_record_offset":11992,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":12476,"parent_record_offset":12448,"ancestor_record_offset":11992,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":12928,"parent_record_offset":12864,"ancestor_record_offset":11992,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":12272,"parent_record_offset":12236,"ancestor_record_offset":11992,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":12952,"parent_record_offset":12928,"ancestor_record_offset":11992,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":11992,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":12836,"parent_record_offset":12808,"ancestor_record_offset":11992,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":12236,"parent_record_offset":12208,"ancestor_record_offset":11992,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":12864,"parent_record_offset":12836,"ancestor_record_offset":11992,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":12328,"parent_record_offset":12236,"ancestor_record_offset":11992,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":12900,"parent_record_offset":12864,"ancestor_record_offset":11992,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$4::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":707,"record_offset":12504,"parent_record_offset":12476,"ancestor_record_offset":11992,"inlinee":39301,"source_line":125,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$9::init_texture_data::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 621
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":707,"record_offset":10984,"rva":12599200,"va":5381308320}
// 编译实例 RVAs: 0xc03fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11150815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e821da55ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381308938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405df600
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6441667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e83db59dff","size":5,"soft_fail":false,"target_va":5374866944,"va":5381308606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ff290
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6311686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e8fab09fff","size":5,"soft_fail":false,"target_va":5374997136,"va":5381308817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6227092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e86cfba0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381308591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bddf00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-156373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e82b9dfdff","size":5,"soft_fail":false,"target_va":5381152512,"va":5381308880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bef4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-84868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e87cb4feff","size":5,"soft_fail":false,"target_va":5381223584,"va":5381308447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30233284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e8c452cd01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381308775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30183633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e8d190cc01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381308730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30186050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e8429acc01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381308745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t515205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e885dc0700","size":5,"soft_fail":false,"target_va":5381823600,"va":5381308390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t515322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e8fadc0700","size":5,"soft_fail":false,"target_va":5381824016,"va":5381308689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::begin
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-93703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e8f991feff","size":5,"soft_fail":false,"target_va":5381214768,"va":5381308466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::impl$9::init_texture_data::closure$1
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381308320,"raw_hex":"e819450000","size":5,"soft_fail":false,"target_va":5381326176,"va":5381308482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":11412,"parent_record_offset":10984,"ancestor_record_offset":10984,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":11440,"parent_record_offset":10984,"ancestor_record_offset":10984,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":11200,"parent_record_offset":10984,"ancestor_record_offset":10984,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":11356,"parent_record_offset":10984,"ancestor_record_offset":10984,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":11740,"parent_record_offset":11716,"ancestor_record_offset":10984,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":11716,"parent_record_offset":11624,"ancestor_record_offset":10984,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":11652,"parent_record_offset":11624,"ancestor_record_offset":10984,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":10984,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":11600,"parent_record_offset":11576,"ancestor_record_offset":10984,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":11380,"parent_record_offset":11356,"ancestor_record_offset":10984,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":707,"record_offset":11684,"parent_record_offset":11652,"ancestor_record_offset":10984,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":11468,"parent_record_offset":11440,"ancestor_record_offset":10984,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":11920,"parent_record_offset":11856,"ancestor_record_offset":10984,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":11264,"parent_record_offset":11228,"ancestor_record_offset":10984,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":11944,"parent_record_offset":11920,"ancestor_record_offset":10984,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":11288,"parent_record_offset":11264,"ancestor_record_offset":10984,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":11828,"parent_record_offset":11800,"ancestor_record_offset":10984,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":11228,"parent_record_offset":11200,"ancestor_record_offset":10984,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":11856,"parent_record_offset":11828,"ancestor_record_offset":10984,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":11320,"parent_record_offset":11228,"ancestor_record_offset":10984,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":11892,"parent_record_offset":11856,"ancestor_record_offset":10984,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$4::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":707,"record_offset":11496,"parent_record_offset":11468,"ancestor_record_offset":10984,"inlinee":39301,"source_line":125,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$9::update_texture_impl::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 621
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":707,"record_offset":13000,"rva":12600672,"va":5381309792}
// 编译实例 RVAs: 0xc04560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11152287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e861d455ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381310410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405df600
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6443139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e87daf9dff","size":5,"soft_fail":false,"target_va":5374866944,"va":5381310078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ff290
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6313158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e83aab9fff","size":5,"soft_fail":false,"target_va":5374997136,"va":5381310289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6228564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e8acf5a0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381310063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bddf00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-157845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e86b97fdff","size":5,"soft_fail":false,"target_va":5381152512,"va":5381310352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bef4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-86340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e8bcaefeff","size":5,"soft_fail":false,"target_va":5381223584,"va":5381309919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30231812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e8044dcd01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381310247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30182161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e8118bcc01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381310202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30184578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e88294cc01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381310217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t513733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e8c5d60700","size":5,"soft_fail":false,"target_va":5381823600,"va":5381309862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t513850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e83ad70700","size":5,"soft_fail":false,"target_va":5381824016,"va":5381310161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::begin
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-95175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e8398cfeff","size":5,"soft_fail":false,"target_va":5381214768,"va":5381309938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::impl$9::update_texture_impl::closure$2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381309792,"raw_hex":"e829420000","size":5,"soft_fail":false,"target_va":5381326896,"va":5381309954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":13428,"parent_record_offset":13000,"ancestor_record_offset":13000,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":13456,"parent_record_offset":13000,"ancestor_record_offset":13000,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":13216,"parent_record_offset":13000,"ancestor_record_offset":13000,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":13372,"parent_record_offset":13000,"ancestor_record_offset":13000,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":13756,"parent_record_offset":13732,"ancestor_record_offset":13000,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":13732,"parent_record_offset":13640,"ancestor_record_offset":13000,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":13668,"parent_record_offset":13640,"ancestor_record_offset":13000,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":13640,"parent_record_offset":13616,"ancestor_record_offset":13000,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":707,"record_offset":13616,"parent_record_offset":13592,"ancestor_record_offset":13000,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":13396,"parent_record_offset":13372,"ancestor_record_offset":13000,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":707,"record_offset":13700,"parent_record_offset":13668,"ancestor_record_offset":13000,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":13484,"parent_record_offset":13456,"ancestor_record_offset":13000,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":13936,"parent_record_offset":13872,"ancestor_record_offset":13000,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":13280,"parent_record_offset":13244,"ancestor_record_offset":13000,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":13960,"parent_record_offset":13936,"ancestor_record_offset":13000,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":13304,"parent_record_offset":13280,"ancestor_record_offset":13000,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":707,"record_offset":13844,"parent_record_offset":13816,"ancestor_record_offset":13000,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":13244,"parent_record_offset":13216,"ancestor_record_offset":13000,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":707,"record_offset":13872,"parent_record_offset":13844,"ancestor_record_offset":13000,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":13336,"parent_record_offset":13244,"ancestor_record_offset":13000,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":707,"record_offset":13908,"parent_record_offset":13872,"ancestor_record_offset":13000,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$4::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":707,"record_offset":13512,"parent_record_offset":13484,"ancestor_record_offset":13000,"inlinee":39301,"source_line":125,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info
// 记录形式: compiled_procedure; instances=1
// 源码行: 647
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":64352,"rva":12551392,"va":5381260512}
// 编译实例 RVAs: 0xbf84e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11103506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381260512,"raw_hex":"e8ee9256ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381261629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bddf00
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-108943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381260512,"raw_hex":"e87156feff","size":5,"soft_fail":false,"target_va":5381152512,"va":5381261450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdf860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-102505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381260512,"raw_hex":"e8976ffeff","size":5,"soft_fail":false,"target_va":5381159008,"va":5381261508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30585999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381260512,"raw_hex":"e88fb4d201","size":5,"soft_fail":false,"target_va":5411847168,"va":5381261164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6903784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381260512,"raw_hex":"e818a896ff","size":5,"soft_fail":false,"target_va":5374357296,"va":5381261075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30280549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381260512,"raw_hex":"e8650bce01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381261350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30281467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381260512,"raw_hex":"e8fb0ece01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381261408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13405519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381260512,"raw_hex":"e84f8dcc00","size":5,"soft_fail":false,"target_va":5394666368,"va":5381260844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30231133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381260512,"raw_hex":"e85d4acd01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381261230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30233550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381260512,"raw_hex":"e8ce53cd01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381261245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t563017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381260512,"raw_hex":"e849970800","size":5,"soft_fail":false,"target_va":5381823600,"va":5381260578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t562822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381260512,"raw_hex":"e886960800","size":5,"soft_fail":false,"target_va":5381824016,"va":5381261189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*992(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381260512,"raw_hex":"ff97e0030000","size":6,"soft_fail":false,"va":5381260804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_pipeline_barrier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":64792,"parent_record_offset":64352,"ancestor_record_offset":64352,"inlinee":39588,"source_line":2370,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":64680,"parent_record_offset":64352,"ancestor_record_offset":64352,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":65336,"parent_record_offset":64352,"ancestor_record_offset":64352,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":64524,"parent_record_offset":64352,"ancestor_record_offset":64352,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBuffer::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":703,"record_offset":64820,"parent_record_offset":64352,"ancestor_record_offset":64352,"inlinee":39502,"source_line":229,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmitInfo::command_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":703,"record_offset":65108,"parent_record_offset":64352,"ancestor_record_offset":64352,"inlinee":39508,"source_line":25,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmitInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":703,"record_offset":65052,"parent_record_offset":64352,"ancestor_record_offset":64352,"inlinee":39505,"source_line":21,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":65212,"parent_record_offset":65188,"ancestor_record_offset":64352,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":65260,"parent_record_offset":65236,"ancestor_record_offset":64352,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":65236,"parent_record_offset":65212,"ancestor_record_offset":64352,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":65188,"parent_record_offset":65148,"ancestor_record_offset":64352,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":65148,"parent_record_offset":65108,"ancestor_record_offset":64352,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::end_command_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":64900,"parent_record_offset":64820,"ancestor_record_offset":64352,"inlinee":39503,"source_line":2423,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":64924,"parent_record_offset":64900,"ancestor_record_offset":64352,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":64948,"parent_record_offset":64924,"ancestor_record_offset":64352,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":703,"record_offset":65020,"parent_record_offset":64820,"ancestor_record_offset":64352,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":703,"record_offset":64704,"parent_record_offset":64680,"ancestor_record_offset":64352,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":64728,"parent_record_offset":64704,"ancestor_record_offset":64352,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":64756,"parent_record_offset":64704,"ancestor_record_offset":64352,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":64984,"parent_record_offset":64820,"ancestor_record_offset":64352,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":65456,"parent_record_offset":65392,"ancestor_record_offset":64352,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":64588,"parent_record_offset":64552,"ancestor_record_offset":64352,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":65480,"parent_record_offset":65456,"ancestor_record_offset":64352,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":64612,"parent_record_offset":64588,"ancestor_record_offset":64352,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":65364,"parent_record_offset":65336,"ancestor_record_offset":64352,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":64552,"parent_record_offset":64524,"ancestor_record_offset":64352,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":65392,"parent_record_offset":65364,"ancestor_record_offset":64352,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":64644,"parent_record_offset":64552,"ancestor_record_offset":64352,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":65428,"parent_record_offset":65392,"ancestor_record_offset":64352,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::end_command_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":64868,"parent_record_offset":64820,"ancestor_record_offset":64352,"inlinee":39351,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":703,"record_offset":65076,"parent_record_offset":65052,"ancestor_record_offset":64352,"inlinee":39507,"source_line":10,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::physical_device
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 678, 679
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A8B
// 首条位置: {"module":702,"inlinee":39563}
// 编译实例 RVAs: 0xbf8950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$77::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":65668,"parent_record_offset":65528,"ancestor_record_offset":65528,"inlinee":7061,"source_line":3917,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":65688,"parent_record_offset":65668,"ancestor_record_offset":65528,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::debug_utils
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=15
// 源码行: 682, 683
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AB2
// 首条位置: {"module":703,"inlinee":39602}
// 编译实例 RVAs: 0xbf8960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":11556,"parent_record_offset":11532,"ancestor_record_offset":11532,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::limits
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 686
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B84
// 首条位置: {"module":705,"inlinee":39812}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::max_bindless_descriptor_count
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 690
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B5C
// 首条位置: {"module":703,"inlinee":39772}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":143796,"parent_record_offset":143740,"ancestor_record_offset":143740,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":143768,"parent_record_offset":143740,"ancestor_record_offset":143740,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::rt
// 记录形式: inlinee_declaration; instances=2
// 源码行: 704
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B86
// 首条位置: {"module":1935,"inlinee":39814}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::wait_idle
// 记录形式: compiled_procedure; instances=1
// 源码行: 708
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":65888,"rva":12552576,"va":5381261696}
// 编译实例 RVAs: 0xbf8980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1382050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e8a2161500","size":5,"soft_fail":false,"target_va":5382644528,"va":5381262473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5746324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e86c51a8ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381262351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30278856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e8c804ce01","size":5,"soft_fail":false,"target_va":5411541584,"va":5381262723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-45444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e87c4effff","size":5,"soft_fail":false,"target_va":5381216320,"va":5381261759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30281526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e8360fce01","size":5,"soft_fail":false,"target_va":5411543360,"va":5381261829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13404044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e88c87cc00","size":5,"soft_fail":false,"target_va":5394666368,"va":5381262319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30222426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e85a28cd01","size":5,"soft_fail":false,"target_va":5411485168,"va":5381262737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30224827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e8bb31cd01","size":5,"soft_fail":false,"target_va":5411487584,"va":5381262752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t561085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e8bd8f0800","size":5,"soft_fail":false,"target_va":5381823600,"va":5381262510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t561465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e839910800","size":5,"soft_fail":false,"target_va":5381824016,"va":5381262546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t123258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e87ae10100","size":5,"soft_fail":false,"target_va":5381385536,"va":5381262273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t128444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e8bcf50100","size":5,"soft_fail":false,"target_va":5381390640,"va":5381262191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t123845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e8c5e30100","size":5,"soft_fail":false,"target_va":5381386480,"va":5381262630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t122969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e859e00100","size":5,"soft_fail":false,"target_va":5381384704,"va":5381261730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30475138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e88203d101","size":5,"soft_fail":false,"target_va":5411737808,"va":5381262665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13410273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e8e19fcc00","size":5,"soft_fail":false,"target_va":5394673152,"va":5381262874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30274388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381261696,"raw_hex":"e854f3cd01","size":5,"soft_fail":false,"target_va":5411537104,"va":5381262711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*152(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381261696,"raw_hex":"ff9698000000","size":6,"soft_fail":false,"va":5381262537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::device_wait_idle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":69000,"parent_record_offset":65888,"ancestor_record_offset":65888,"inlinee":39709,"source_line":1578,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":68576,"parent_record_offset":65888,"ancestor_record_offset":65888,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":703,"record_offset":69028,"parent_record_offset":65888,"ancestor_record_offset":65888,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":69308,"parent_record_offset":65888,"ancestor_record_offset":65888,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":703,"record_offset":68784,"parent_record_offset":65888,"ancestor_record_offset":65888,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":703,"record_offset":68608,"parent_record_offset":65888,"ancestor_record_offset":65888,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":68492,"parent_record_offset":65888,"ancestor_record_offset":65888,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":67492,"parent_record_offset":65888,"ancestor_record_offset":65888,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":66060,"parent_record_offset":65888,"ancestor_record_offset":65888,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":66516,"parent_record_offset":65888,"ancestor_record_offset":65888,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::wait_idle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":703,"record_offset":68460,"parent_record_offset":65888,"ancestor_record_offset":65888,"inlinee":39332,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":67812,"parent_record_offset":67784,"ancestor_record_offset":65888,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":67864,"parent_record_offset":67840,"ancestor_record_offset":65888,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":67840,"parent_record_offset":67812,"ancestor_record_offset":65888,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":67760,"parent_record_offset":67732,"ancestor_record_offset":65888,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":67784,"parent_record_offset":67760,"ancestor_record_offset":65888,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":67956,"parent_record_offset":67784,"ancestor_record_offset":65888,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":68068,"parent_record_offset":68044,"ancestor_record_offset":65888,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":703,"record_offset":67888,"parent_record_offset":67864,"ancestor_record_offset":65888,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":703,"record_offset":66216,"parent_record_offset":66184,"ancestor_record_offset":65888,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":703,"record_offset":67628,"parent_record_offset":67588,"ancestor_record_offset":65888,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":703,"record_offset":68664,"parent_record_offset":68632,"ancestor_record_offset":65888,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":703,"record_offset":68692,"parent_record_offset":68632,"ancestor_record_offset":65888,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":66464,"parent_record_offset":66436,"ancestor_record_offset":65888,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":68416,"parent_record_offset":67588,"ancestor_record_offset":65888,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":68020,"parent_record_offset":67732,"ancestor_record_offset":65888,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":66928,"parent_record_offset":66904,"ancestor_record_offset":65888,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":66904,"parent_record_offset":66880,"ancestor_record_offset":65888,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":67112,"parent_record_offset":67084,"ancestor_record_offset":65888,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":67912,"parent_record_offset":67888,"ancestor_record_offset":65888,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":66856,"parent_record_offset":66832,"ancestor_record_offset":65888,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":66880,"parent_record_offset":66856,"ancestor_record_offset":65888,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":67184,"parent_record_offset":67156,"ancestor_record_offset":65888,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":66832,"parent_record_offset":66716,"ancestor_record_offset":65888,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":703,"record_offset":66184,"parent_record_offset":66156,"ancestor_record_offset":65888,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":703,"record_offset":66544,"parent_record_offset":66516,"ancestor_record_offset":65888,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":703,"record_offset":66684,"parent_record_offset":66652,"ancestor_record_offset":65888,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":703,"record_offset":66652,"parent_record_offset":66616,"ancestor_record_offset":65888,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":66592,"parent_record_offset":66568,"ancestor_record_offset":65888,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":66616,"parent_record_offset":66592,"ancestor_record_offset":65888,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":66568,"parent_record_offset":66544,"ancestor_record_offset":65888,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":703,"record_offset":67156,"parent_record_offset":66616,"ancestor_record_offset":65888,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":66156,"parent_record_offset":66124,"ancestor_record_offset":65888,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":66124,"parent_record_offset":66096,"ancestor_record_offset":65888,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":66316,"parent_record_offset":66284,"ancestor_record_offset":65888,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":66412,"parent_record_offset":66316,"ancestor_record_offset":65888,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":66284,"parent_record_offset":66260,"ancestor_record_offset":65888,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":66096,"parent_record_offset":66060,"ancestor_record_offset":65888,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":66436,"parent_record_offset":66412,"ancestor_record_offset":65888,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":66716,"parent_record_offset":66684,"ancestor_record_offset":65888,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":67000,"parent_record_offset":66716,"ancestor_record_offset":65888,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":66972,"parent_record_offset":66716,"ancestor_record_offset":65888,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":703,"record_offset":66776,"parent_record_offset":66716,"ancestor_record_offset":65888,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":69236,"parent_record_offset":69112,"ancestor_record_offset":65888,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":68848,"parent_record_offset":68812,"ancestor_record_offset":65888,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":68516,"parent_record_offset":68492,"ancestor_record_offset":65888,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":69260,"parent_record_offset":69236,"ancestor_record_offset":65888,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":68872,"parent_record_offset":68848,"ancestor_record_offset":65888,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":68540,"parent_record_offset":68516,"ancestor_record_offset":65888,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":703,"record_offset":69084,"parent_record_offset":69056,"ancestor_record_offset":65888,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":68720,"parent_record_offset":68632,"ancestor_record_offset":65888,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":703,"record_offset":68632,"parent_record_offset":68608,"ancestor_record_offset":65888,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":68744,"parent_record_offset":68720,"ancestor_record_offset":65888,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":703,"record_offset":68904,"parent_record_offset":68812,"ancestor_record_offset":65888,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":703,"record_offset":69148,"parent_record_offset":69112,"ancestor_record_offset":65888,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":703,"record_offset":68812,"parent_record_offset":68784,"ancestor_record_offset":65888,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":703,"record_offset":69112,"parent_record_offset":69084,"ancestor_record_offset":65888,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":68932,"parent_record_offset":68904,"ancestor_record_offset":65888,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":69176,"parent_record_offset":69148,"ancestor_record_offset":65888,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":67524,"parent_record_offset":67492,"ancestor_record_offset":65888,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":69372,"parent_record_offset":69340,"ancestor_record_offset":65888,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":69872,"parent_record_offset":69612,"ancestor_record_offset":65888,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":703,"record_offset":67656,"parent_record_offset":67628,"ancestor_record_offset":65888,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":67732,"parent_record_offset":67588,"ancestor_record_offset":65888,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":703,"record_offset":67992,"parent_record_offset":67732,"ancestor_record_offset":65888,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":703,"record_offset":67684,"parent_record_offset":67656,"ancestor_record_offset":65888,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":703,"record_offset":67588,"parent_record_offset":67556,"ancestor_record_offset":65888,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":703,"record_offset":67556,"parent_record_offset":67524,"ancestor_record_offset":65888,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 725
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":70464,"rva":12553824,"va":5381262944}
// 编译实例 RVAs: 0xbf8e60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11105082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381262944,"raw_hex":"e8c68c56ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381263205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30229212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381262944,"raw_hex":"e8dc42cd01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381263151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t560575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381262944,"raw_hex":"e8bf8d0800","size":5,"soft_fail":false,"target_va":5381823600,"va":5381263020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t560893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381262944,"raw_hex":"e8fd8e0800","size":5,"soft_fail":false,"target_va":5381824016,"va":5381263118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381262944,"raw_hex":"e803fbffff","size":5,"soft_fail":false,"target_va":5381261696,"va":5381262968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::Queue::reclaim_command_buffer_resources
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381262944,"raw_hex":"e8f07dffff","size":5,"soft_fail":false,"target_va":5381229760,"va":5381263051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13401060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381262944,"raw_hex":"e8e47bcc00","size":5,"soft_fail":false,"target_va":5394664048,"va":5381262983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13411391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381262944,"raw_hex":"e83fa4cc00","size":5,"soft_fail":false,"target_va":5394674432,"va":5381263036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t30231621","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381262944,"raw_hex":"e9454ccd01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381263174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":71128,"parent_record_offset":70464,"ancestor_record_offset":70464,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":70632,"parent_record_offset":70464,"ancestor_record_offset":70464,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":70788,"parent_record_offset":70464,"ancestor_record_offset":70464,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::reclaim_command_buffer_resources_all_queues
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":703,"record_offset":70844,"parent_record_offset":70464,"ancestor_record_offset":70464,"inlinee":39307,"source_line":135,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":70924,"parent_record_offset":70900,"ancestor_record_offset":70464,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":70876,"parent_record_offset":70844,"ancestor_record_offset":70464,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":70900,"parent_record_offset":70876,"ancestor_record_offset":70464,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":70812,"parent_record_offset":70788,"ancestor_record_offset":70464,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":71084,"parent_record_offset":71060,"ancestor_record_offset":70464,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":71004,"parent_record_offset":70960,"ancestor_record_offset":70464,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":71060,"parent_record_offset":71036,"ancestor_record_offset":70464,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":70960,"parent_record_offset":70844,"ancestor_record_offset":70464,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":71036,"parent_record_offset":70960,"ancestor_record_offset":70464,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":71248,"parent_record_offset":71184,"ancestor_record_offset":70464,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":70696,"parent_record_offset":70660,"ancestor_record_offset":70464,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":71272,"parent_record_offset":71248,"ancestor_record_offset":70464,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":70720,"parent_record_offset":70696,"ancestor_record_offset":70464,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":71156,"parent_record_offset":71128,"ancestor_record_offset":70464,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":70660,"parent_record_offset":70632,"ancestor_record_offset":70464,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":71184,"parent_record_offset":71156,"ancestor_record_offset":70464,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":70752,"parent_record_offset":70660,"ancestor_record_offset":70464,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":71220,"parent_record_offset":71184,"ancestor_record_offset":70464,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::validate_physical_device_requirements
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 736
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9ACC
// 首条位置: {"module":703,"inlinee":39628}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":703,"record_offset":43872,"parent_record_offset":43832,"ancestor_record_offset":43832,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":703,"record_offset":43928,"parent_record_offset":43900,"ancestor_record_offset":43832,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":43900,"parent_record_offset":43872,"ancestor_record_offset":43832,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 758
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1657,"record_offset":9744,"rva":23737888,"va":5392447008}
// 编译实例 RVAs: 0x16a3620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22289118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447008,"raw_hex":"e822e5abfe","size":5,"soft_fail":false,"target_va":5370158128,"va":5392447241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a4ca70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12938279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447008,"raw_hex":"e8d9933aff","size":5,"soft_fail":false,"target_va":5379508848,"va":5392447122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19045178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447008,"raw_hex":"e83a9b2201","size":5,"soft_fail":false,"target_va":5411492368,"va":5392447185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19047595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447008,"raw_hex":"e8aba42201","size":5,"soft_fail":false,"target_va":5411494800,"va":5392447200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10623470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447008,"raw_hex":"e812e65dff","size":5,"soft_fail":false,"target_va":5381823600,"va":5392447065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10623134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447008,"raw_hex":"e862e75dff","size":5,"soft_fail":false,"target_va":5381824016,"va":5392447145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::peak_allocator_report
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11096187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447008,"raw_hex":"e885af56ff","size":5,"soft_fail":false,"target_va":5381350928,"va":5392447110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::peak_device_local_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11096122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447008,"raw_hex":"e8c6af56ff","size":5,"soft_fail":false,"target_va":5381350960,"va":5392447077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::peak_system_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11096117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447008,"raw_hex":"e8cbaf56ff","size":5,"soft_fail":false,"target_va":5381350976,"va":5392447088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::peak_total_allocated_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11096192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392447008,"raw_hex":"e880af56ff","size":5,"soft_fail":false,"target_va":5381350912,"va":5392447099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1657,"record_offset":10064,"parent_record_offset":9744,"ancestor_record_offset":9744,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":10348,"parent_record_offset":9744,"ancestor_record_offset":9744,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] country_core::meta_commander::commander_update::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/meta_commander.rs
//     evidence={"module":1657,"record_offset":10304,"parent_record_offset":9744,"ancestor_record_offset":9744,"inlinee":75158,"source_line":367,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1657,"record_offset":10092,"parent_record_offset":9744,"ancestor_record_offset":9744,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1657,"record_offset":10248,"parent_record_offset":9744,"ancestor_record_offset":9744,"inlinee":15806,"source_line":639,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":9744,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":10468,"parent_record_offset":10404,"ancestor_record_offset":9744,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":10156,"parent_record_offset":10120,"ancestor_record_offset":9744,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":10492,"parent_record_offset":10468,"ancestor_record_offset":9744,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":10180,"parent_record_offset":10156,"ancestor_record_offset":9744,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1657,"record_offset":10376,"parent_record_offset":10348,"ancestor_record_offset":9744,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1657,"record_offset":10120,"parent_record_offset":10092,"ancestor_record_offset":9744,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1657,"record_offset":10404,"parent_record_offset":10376,"ancestor_record_offset":9744,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1657,"record_offset":10212,"parent_record_offset":10120,"ancestor_record_offset":9744,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1657,"record_offset":10440,"parent_record_offset":10404,"ancestor_record_offset":9744,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::game_loop::shut_down_everything::closure_env$0,tuple$<u64,u64> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 758
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":245,"record_offset":28832,"rva":6003248,"va":5374712368}
// 编译实例 RVAs: 0x5b9a30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4554426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374712368,"raw_hex":"e84681baff","size":5,"soft_fail":false,"target_va":5370158128,"va":5374712549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36779868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374712368,"raw_hex":"e85c373102","size":5,"soft_fail":false,"target_va":5411492368,"va":5374712495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36782288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374712368,"raw_hex":"e8d0403102","size":5,"soft_fail":false,"target_va":5411494800,"va":5374712507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7111177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374712368,"raw_hex":"e809826c00","size":5,"soft_fail":false,"target_va":5381823600,"va":5374712418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7111555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374712368,"raw_hex":"e883836c00","size":5,"soft_fail":false,"target_va":5381824016,"va":5374712456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::peak_device_local_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6638525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374712368,"raw_hex":"e8bd4b6500","size":5,"soft_fail":false,"target_va":5381350960,"va":5374712430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::peak_system_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6638530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374712368,"raw_hex":"e8c24b6500","size":5,"soft_fail":false,"target_va":5381350976,"va":5374712441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":245,"record_offset":29072,"parent_record_offset":28832,"ancestor_record_offset":28832,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":245,"record_offset":29340,"parent_record_offset":28832,"ancestor_record_offset":28832,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":245,"record_offset":29100,"parent_record_offset":28832,"ancestor_record_offset":28832,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":245,"record_offset":29256,"parent_record_offset":28832,"ancestor_record_offset":28832,"inlinee":15806,"source_line":639,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::shut_down_everything::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":245,"record_offset":29312,"parent_record_offset":28832,"ancestor_record_offset":28832,"inlinee":15811,"source_line":1034,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":245,"record_offset":29280,"parent_record_offset":29256,"ancestor_record_offset":28832,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":245,"record_offset":29452,"parent_record_offset":29392,"ancestor_record_offset":28832,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":245,"record_offset":29164,"parent_record_offset":29128,"ancestor_record_offset":28832,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":245,"record_offset":29476,"parent_record_offset":29452,"ancestor_record_offset":28832,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":245,"record_offset":29188,"parent_record_offset":29164,"ancestor_record_offset":28832,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":245,"record_offset":29364,"parent_record_offset":29340,"ancestor_record_offset":28832,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":245,"record_offset":29128,"parent_record_offset":29100,"ancestor_record_offset":28832,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":245,"record_offset":29392,"parent_record_offset":29364,"ancestor_record_offset":28832,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":245,"record_offset":29220,"parent_record_offset":29128,"ancestor_record_offset":28832,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":245,"record_offset":29424,"parent_record_offset":29392,"ancestor_record_offset":28832,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure_env$0,tuple$<> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 758
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":245,"record_offset":27552,"rva":6002704,"va":5374711824}
// 编译实例 RVAs: 0x5b9810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140053a80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5660509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e8a3a0a9ff","size":5,"soft_fail":false,"target_va":5369051776,"va":5374712280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140053bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5659890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e80ea3a9ff","size":5,"soft_fail":false,"target_va":5369052080,"va":5374711965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140053ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5659182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e8d2a5a9ff","size":5,"soft_fail":false,"target_va":5369052896,"va":5374712073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4554214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e81a82baff","size":5,"soft_fail":false,"target_va":5370158128,"va":5374712337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t369467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e83ba30500","size":5,"soft_fail":false,"target_va":5375081488,"va":5374712016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t804104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e808450c00","size":5,"soft_fail":false,"target_va":5375516032,"va":5374711923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37135169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e841a33602","size":5,"soft_fail":false,"target_va":5411847168,"va":5374711994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36828769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e861f63102","size":5,"soft_fail":false,"target_va":5411540992,"va":5374712218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6846528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e840786800","size":5,"soft_fail":false,"target_va":5381558640,"va":5374712107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::visualizer::AllocatorVisualizer::render_breakdown_window
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6669463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e897c46500","size":5,"soft_fail":false,"target_va":5381381408,"va":5374711940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::visualizer::AllocatorVisualizer::render_memory_block_visualization_windows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6668890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e85ac26500","size":5,"soft_fail":false,"target_va":5381381024,"va":5374712129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36780177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e891383102","size":5,"soft_fail":false,"target_va":5411492368,"va":5374712186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36782594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e802423102","size":5,"soft_fail":false,"target_va":5411494800,"va":5374712201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7111684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e804846c00","size":5,"soft_fail":false,"target_va":5381823600,"va":5374711911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7111867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374711824,"raw_hex":"e8bb846c00","size":5,"soft_fail":false,"target_va":5381824016,"va":5374712144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":245,"record_offset":27792,"parent_record_offset":27552,"ancestor_record_offset":27552,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":245,"record_offset":28640,"parent_record_offset":27552,"ancestor_record_offset":27552,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":245,"record_offset":27820,"parent_record_offset":27552,"ancestor_record_offset":27552,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":245,"record_offset":27976,"parent_record_offset":27552,"ancestor_record_offset":27552,"inlinee":15806,"source_line":639,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_debug.rs
//     evidence={"module":245,"record_offset":28032,"parent_record_offset":27552,"ancestor_record_offset":27552,"inlinee":15808,"source_line":958,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":245,"record_offset":28224,"parent_record_offset":28200,"ancestor_record_offset":27552,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":245,"record_offset":28248,"parent_record_offset":28224,"ancestor_record_offset":27552,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":245,"record_offset":28164,"parent_record_offset":28136,"ancestor_record_offset":27552,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":245,"record_offset":28200,"parent_record_offset":28164,"ancestor_record_offset":27552,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":245,"record_offset":28136,"parent_record_offset":28104,"ancestor_record_offset":27552,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":245,"record_offset":28492,"parent_record_offset":28456,"ancestor_record_offset":27552,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":245,"record_offset":28456,"parent_record_offset":28428,"ancestor_record_offset":27552,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":245,"record_offset":28000,"parent_record_offset":27976,"ancestor_record_offset":27552,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":245,"record_offset":28328,"parent_record_offset":28032,"ancestor_record_offset":27552,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":245,"record_offset":28524,"parent_record_offset":28492,"ancestor_record_offset":27552,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":245,"record_offset":28276,"parent_record_offset":28248,"ancestor_record_offset":27552,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":245,"record_offset":28760,"parent_record_offset":28696,"ancestor_record_offset":27552,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":245,"record_offset":27884,"parent_record_offset":27848,"ancestor_record_offset":27552,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":245,"record_offset":28784,"parent_record_offset":28760,"ancestor_record_offset":27552,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":245,"record_offset":27908,"parent_record_offset":27884,"ancestor_record_offset":27552,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":245,"record_offset":28552,"parent_record_offset":28492,"ancestor_record_offset":27552,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":245,"record_offset":28584,"parent_record_offset":28456,"ancestor_record_offset":27552,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::window::Window::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":245,"record_offset":28076,"parent_record_offset":28032,"ancestor_record_offset":27552,"inlinee":15809,"source_line":81,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::window::Window::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":245,"record_offset":28104,"parent_record_offset":28032,"ancestor_record_offset":27552,"inlinee":9371,"source_line":365,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":245,"record_offset":28668,"parent_record_offset":28640,"ancestor_record_offset":27552,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":245,"record_offset":27848,"parent_record_offset":27820,"ancestor_record_offset":27552,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":245,"record_offset":28696,"parent_record_offset":28668,"ancestor_record_offset":27552,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":245,"record_offset":27940,"parent_record_offset":27848,"ancestor_record_offset":27552,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":245,"record_offset":28732,"parent_record_offset":28696,"ancestor_record_offset":27552,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_global_allocator<toolbar::menu::report_bug::ui::closure_env$10,gpu_allocator::allocator::AllocatorReport>
// 记录形式: compiled_procedure; instances=1
// 源码行: 758
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1189,"record_offset":864,"rva":16444128,"va":5385153248}
// 编译实例 RVAs: 0xfaeae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14995290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385153248,"raw_hex":"e8a6301bff","size":5,"soft_fail":false,"target_va":5370158128,"va":5385153413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9637286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385153248,"raw_hex":"e85af26cff","size":5,"soft_fail":false,"target_va":5375516032,"va":5385153313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocator::generate_report
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3774945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385153248,"raw_hex":"e81f66c6ff","size":5,"soft_fail":false,"target_va":5381378384,"va":5385153324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t26338991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385153248,"raw_hex":"e8afe69101","size":5,"soft_fail":false,"target_va":5411492368,"va":5385153372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t26341411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385153248,"raw_hex":"e823f09101","size":5,"soft_fail":false,"target_va":5411494800,"va":5385153384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-3329706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385153248,"raw_hex":"e85631cdff","size":5,"soft_fail":false,"target_va":5381823600,"va":5385153301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-3329325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385153248,"raw_hex":"e8d332cdff","size":5,"soft_fail":false,"target_va":5381824016,"va":5385153336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1189,"record_offset":1116,"parent_record_offset":864,"ancestor_record_offset":864,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1189,"record_offset":1380,"parent_record_offset":864,"ancestor_record_offset":864,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1189,"record_offset":1144,"parent_record_offset":864,"ancestor_record_offset":864,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1189,"record_offset":1300,"parent_record_offset":864,"ancestor_record_offset":864,"inlinee":15806,"source_line":639,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::report_bug::ui::closure$10
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/report_bug.rs
//     evidence={"module":1189,"record_offset":1356,"parent_record_offset":864,"ancestor_record_offset":864,"inlinee":61938,"source_line":323,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1189,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":864,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1189,"record_offset":1492,"parent_record_offset":1432,"ancestor_record_offset":864,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1189,"record_offset":1208,"parent_record_offset":1172,"ancestor_record_offset":864,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1189,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":864,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1189,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":864,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1189,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":864,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1189,"record_offset":1172,"parent_record_offset":1144,"ancestor_record_offset":864,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1189,"record_offset":1432,"parent_record_offset":1404,"ancestor_record_offset":864,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1189,"record_offset":1264,"parent_record_offset":1172,"ancestor_record_offset":864,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1189,"record_offset":1464,"parent_record_offset":1432,"ancestor_record_offset":864,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01189.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
// 记录形式: compiled_procedure; instances=1
// 源码行: 766
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":71320,"rva":12554112,"va":5381263232}
// 编译实例 RVAs: 0xbf8f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] rhapsody::raw::physical_device::PhysicalDevice::refresh_memory_budget
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t34442","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381263232,"raw_hex":"e98a860000","size":5,"soft_fail":false,"target_va":5381297696,"va":5381263249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":71464,"parent_record_offset":71320,"ancestor_record_offset":71320,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::min_subgroup_size
// 记录形式: compiled_procedure; instances=1
// 源码行: 790
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":71492,"rva":12554144,"va":5381263264}
// 编译实例 RVAs: 0xbf8fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::create_raw_device
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 795
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9ADC
// 首条位置: {"module":703,"inlinee":39644}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_features2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":703,"record_offset":51408,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":39654,"source_line":151,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DeviceCreateInfo::push_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":51932,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":39656,"source_line":1383,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceFeatures2::push_next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":51000,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":39652,"source_line":11056,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$2798::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":50944,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":39648,"source_line":30394,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$2906::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":50916,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":39646,"source_line":31432,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$646::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":50972,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":39650,"source_line":11031,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":52340,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":52384,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":703,"record_offset":51148,"parent_record_offset":51124,"ancestor_record_offset":50740,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":703,"record_offset":51052,"parent_record_offset":51028,"ancestor_record_offset":50740,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":703,"record_offset":51076,"parent_record_offset":51052,"ancestor_record_offset":50740,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":703,"record_offset":51124,"parent_record_offset":51100,"ancestor_record_offset":50740,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":51028,"parent_record_offset":51000,"ancestor_record_offset":50740,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":51100,"parent_record_offset":51076,"ancestor_record_offset":50740,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::query_device_extension_support
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1041
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9ACE
// 首条位置: {"module":703,"inlinee":39630}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":44176,"parent_record_offset":44028,"ancestor_record_offset":44028,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":44328,"parent_record_offset":44028,"ancestor_record_offset":44028,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":46792,"parent_record_offset":44028,"ancestor_record_offset":44028,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44412,"parent_record_offset":44028,"ancestor_record_offset":44028,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44804,"parent_record_offset":44028,"ancestor_record_offset":44028,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44440,"parent_record_offset":44028,"ancestor_record_offset":44028,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":703,"record_offset":46536,"parent_record_offset":44028,"ancestor_record_offset":44028,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":703,"record_offset":45256,"parent_record_offset":44028,"ancestor_record_offset":44028,"inlinee":12585,"source_line":264,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":46564,"parent_record_offset":44028,"ancestor_record_offset":44028,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":44700,"parent_record_offset":44028,"ancestor_record_offset":44028,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":703,"record_offset":45352,"parent_record_offset":44028,"ancestor_record_offset":44028,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":44232,"parent_record_offset":44208,"ancestor_record_offset":44028,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":44256,"parent_record_offset":44232,"ancestor_record_offset":44028,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":45196,"parent_record_offset":45168,"ancestor_record_offset":44028,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":44208,"parent_record_offset":44176,"ancestor_record_offset":44028,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":45168,"parent_record_offset":45104,"ancestor_record_offset":44028,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":44544,"parent_record_offset":44520,"ancestor_record_offset":44028,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":45136,"parent_record_offset":45104,"ancestor_record_offset":44028,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":45104,"parent_record_offset":45076,"ancestor_record_offset":44028,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":44592,"parent_record_offset":44568,"ancestor_record_offset":44028,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":44568,"parent_record_offset":44544,"ancestor_record_offset":44028,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":45076,"parent_record_offset":45052,"ancestor_record_offset":44028,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":44352,"parent_record_offset":44328,"ancestor_record_offset":44028,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44856,"parent_record_offset":44804,"ancestor_record_offset":44028,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44520,"parent_record_offset":44496,"ancestor_record_offset":44028,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44496,"parent_record_offset":44464,"ancestor_record_offset":44028,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44376,"parent_record_offset":44352,"ancestor_record_offset":44028,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":44464,"parent_record_offset":44440,"ancestor_record_offset":44028,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":46180,"parent_record_offset":46152,"ancestor_record_offset":44028,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":703,"record_offset":45680,"parent_record_offset":45656,"ancestor_record_offset":44028,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":703,"record_offset":45764,"parent_record_offset":45740,"ancestor_record_offset":44028,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":703,"record_offset":45284,"parent_record_offset":45256,"ancestor_record_offset":44028,"inlinee":12587,"source_line":721,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":45312,"parent_record_offset":45284,"ancestor_record_offset":44028,"inlinee":12589,"source_line":3874,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":45704,"parent_record_offset":45680,"ancestor_record_offset":44028,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":46356,"parent_record_offset":45544,"ancestor_record_offset":44028,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":46952,"parent_record_offset":46920,"ancestor_record_offset":44028,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":703,"record_offset":45912,"parent_record_offset":45884,"ancestor_record_offset":44028,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":703,"record_offset":45884,"parent_record_offset":45856,"ancestor_record_offset":44028,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":45004,"parent_record_offset":44804,"ancestor_record_offset":44028,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":44660,"parent_record_offset":44636,"ancestor_record_offset":44028,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":46032,"parent_record_offset":46008,"ancestor_record_offset":44028,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":44760,"parent_record_offset":44700,"ancestor_record_offset":44028,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":44284,"parent_record_offset":44256,"ancestor_record_offset":44028,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":44976,"parent_record_offset":44804,"ancestor_record_offset":44028,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":46264,"parent_record_offset":46236,"ancestor_record_offset":44028,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":703,"record_offset":46292,"parent_record_offset":46264,"ancestor_record_offset":44028,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":703,"record_offset":44636,"parent_record_offset":44464,"ancestor_record_offset":44028,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":703,"record_offset":46208,"parent_record_offset":46180,"ancestor_record_offset":44028,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":703,"record_offset":45824,"parent_record_offset":45796,"ancestor_record_offset":44028,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":703,"record_offset":45856,"parent_record_offset":45824,"ancestor_record_offset":44028,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":703,"record_offset":45796,"parent_record_offset":45544,"ancestor_record_offset":44028,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":703,"record_offset":45656,"parent_record_offset":45544,"ancestor_record_offset":44028,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":703,"record_offset":46384,"parent_record_offset":45544,"ancestor_record_offset":44028,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":703,"record_offset":45740,"parent_record_offset":45544,"ancestor_record_offset":44028,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":703,"record_offset":45628,"parent_record_offset":45544,"ancestor_record_offset":44028,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":703,"record_offset":46152,"parent_record_offset":46124,"ancestor_record_offset":44028,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":45408,"parent_record_offset":45380,"ancestor_record_offset":44028,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":45436,"parent_record_offset":45408,"ancestor_record_offset":44028,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":46124,"parent_record_offset":45956,"ancestor_record_offset":44028,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":703,"record_offset":45468,"parent_record_offset":45436,"ancestor_record_offset":44028,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":46064,"parent_record_offset":45980,"ancestor_record_offset":44028,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":46008,"parent_record_offset":45980,"ancestor_record_offset":44028,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":46476,"parent_record_offset":45544,"ancestor_record_offset":44028,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":45980,"parent_record_offset":45956,"ancestor_record_offset":44028,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":45520,"parent_record_offset":45496,"ancestor_record_offset":44028,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":45496,"parent_record_offset":45436,"ancestor_record_offset":44028,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":45544,"parent_record_offset":45520,"ancestor_record_offset":44028,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":45600,"parent_record_offset":45544,"ancestor_record_offset":44028,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":703,"record_offset":45956,"parent_record_offset":45544,"ancestor_record_offset":44028,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":703,"record_offset":45380,"parent_record_offset":45352,"ancestor_record_offset":44028,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$5::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 1173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":690,"record_offset":102280,"rva":12460960,"va":5381170080}
// 编译实例 RVAs: 0xbe23a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11012202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170080,"raw_hex":"e896f757ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381170325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30322092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170080,"raw_hex":"e8acadce01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381170271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t653443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170080,"raw_hex":"e883f80900","size":5,"soft_fail":false,"target_va":5381823600,"va":5381170152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t653773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170080,"raw_hex":"e8cdf90900","size":5,"soft_fail":false,"target_va":5381824016,"va":5381170238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t91587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170080,"raw_hex":"e8c3650100","size":5,"soft_fail":false,"target_va":5381261696,"va":5381170104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::Queue::reclaim_command_buffer_resources
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t59588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170080,"raw_hex":"e8c4e80000","size":5,"soft_fail":false,"target_va":5381229760,"va":5381170167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13493924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170080,"raw_hex":"e8a4e6cd00","size":5,"soft_fail":false,"target_va":5394664048,"va":5381170119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t30324501","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381170080,"raw_hex":"e915b7ce01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381170294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":690,"record_offset":102848,"parent_record_offset":102280,"ancestor_record_offset":102280,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":690,"record_offset":102408,"parent_record_offset":102280,"ancestor_record_offset":102280,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::reclaim_command_buffer_resources_all_queues
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":690,"record_offset":102564,"parent_record_offset":102280,"ancestor_record_offset":102280,"inlinee":39307,"source_line":135,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":690,"record_offset":102644,"parent_record_offset":102620,"ancestor_record_offset":102280,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":690,"record_offset":102596,"parent_record_offset":102564,"ancestor_record_offset":102280,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":690,"record_offset":102620,"parent_record_offset":102596,"ancestor_record_offset":102280,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":690,"record_offset":102804,"parent_record_offset":102780,"ancestor_record_offset":102280,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":690,"record_offset":102724,"parent_record_offset":102680,"ancestor_record_offset":102280,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":690,"record_offset":102780,"parent_record_offset":102756,"ancestor_record_offset":102280,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":690,"record_offset":102680,"parent_record_offset":102564,"ancestor_record_offset":102280,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":690,"record_offset":102756,"parent_record_offset":102680,"ancestor_record_offset":102280,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":690,"record_offset":102968,"parent_record_offset":102904,"ancestor_record_offset":102280,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":690,"record_offset":102472,"parent_record_offset":102436,"ancestor_record_offset":102280,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":690,"record_offset":102992,"parent_record_offset":102968,"ancestor_record_offset":102280,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":690,"record_offset":102496,"parent_record_offset":102472,"ancestor_record_offset":102280,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":690,"record_offset":102876,"parent_record_offset":102848,"ancestor_record_offset":102280,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":690,"record_offset":102436,"parent_record_offset":102408,"ancestor_record_offset":102280,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":690,"record_offset":102904,"parent_record_offset":102876,"ancestor_record_offset":102280,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":690,"record_offset":102528,"parent_record_offset":102436,"ancestor_record_offset":102280,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":690,"record_offset":102940,"parent_record_offset":102904,"ancestor_record_offset":102280,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00690.jsonl.gz"}
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
// 完整结构及原始类型记录: device.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::device::DeviceCreateError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::device::DeviceCreateError>", "constructor": "enum2$", "arguments": ["rhapsody::raw::device::DeviceCreateError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<enum2$<rhapsody::raw::device::DeviceCreateError> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["enum2$<rhapsody::raw::device::DeviceCreateError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::device::GlobalQueueLock
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLockReadGuard<parking_lot::raw_rwlock::RawRwLock,rhapsody::raw::device::GlobalQueueLock>", "constructor": "lock_api::rwlock::RwLockReadGuard", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "rhapsody::raw::device::GlobalQueueLock"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::rwlock::RwLockReadGuard<parking_lot::raw_rwlock::RawRwLock,rhapsody::raw::device::GlobalQueueLock> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::rwlock::RwLockReadGuard<parking_lot::raw_rwlock::RawRwLock,rhapsody::raw::device::GlobalQueueLock>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::device::GraphicsDevice
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 76; 过程引用数: 99
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >", "constructor": "rhapsody::raw::device::GraphicsDevice::create_buffer", "arguments": ["enum2$<core::option::Option<ref$<slice2$<u8> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure_env$0,tuple$<> >", "constructor": "rhapsody::raw::device::GraphicsDevice::with_global_allocator", "arguments": ["tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::game_loop::shut_down_everything::closure_env$0,tuple$<u64,u64> >", "constructor": "rhapsody::raw::device::GraphicsDevice::with_global_allocator", "arguments": ["tiny_glade::game_loop::shut_down_everything::closure_env$0", "tuple$<u64,u64>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::create_texture", "arguments": ["ref$<slice2$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> >", "constructor": "rhapsody::raw::device::GraphicsDevice::create_texture", "arguments": ["enum2$<rhapsody::raw::texture::TextureInitialData>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::device::GraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::raw::device::GraphicsDevice>", "constructor": "ref$", "arguments": ["rhapsody::raw::device::GraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>", "constructor": "tuple$", "arguments": ["ref$<rhapsody::raw::device::GraphicsDevice>", "usize", "usize"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::external::closure_env$0,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::external::closure_env$0", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<rhapsody::raw::device::GraphicsDevice,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["rhapsody::raw::device::GraphicsDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::create_buffer<tuple$<> >", "constructor": "rhapsody::raw::device::GraphicsDevice::create_buffer", "arguments": ["tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$9::init_texture_data::closure_env$1>", "constructor": "rhapsody::raw::device::GraphicsDevice::with_setup_cb", "arguments": ["rhapsody::raw::texture::impl$9::init_texture_data::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure_env$0>", "constructor": "rhapsody::raw::device::GraphicsDevice::with_setup_cb", "arguments": ["rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$9::update_texture_impl::closure_env$2>", "constructor": "rhapsody::raw::device::GraphicsDevice::with_setup_cb", "arguments": ["rhapsody::raw::texture::impl$9::update_texture_impl::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > > >", "constructor": "rhapsody::raw::device::GraphicsDevice::with_setup_cb", "arguments": ["rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>", "constructor": "rhapsody::raw::device::GraphicsDevice::queue_submit", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>", "rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_global_allocator<toolbar::menu::report_bug::ui::closure_env$10,gpu_allocator::allocator::AllocatorReport>", "constructor": "rhapsody::raw::device::GraphicsDevice::with_global_allocator", "arguments": ["toolbar::menu::report_bug::ui::closure_env$10", "gpu_allocator::allocator::AllocatorReport"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::BalconyDoorInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::BalconyDoorInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::UberInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::UberInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketCurvedHalftimber>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketCurvedHalftimber>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::StoneFloorInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::StoneFloorInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::SmokeInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketSmoke>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::SmokeInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketSmoke>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::resources::placement_data::PlacementDataSsbo>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidVertexColorInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidVertexColorInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::StairsFloorInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketStairsFloor>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::StairsFloorInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketStairsFloor>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::ClothInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::ClothInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::VelocityOnlyInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::VelocityOnlyInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::PlantInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::PlantInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidAlbedoMapEmissiveInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidAlbedoMapEmissiveInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::PlasterInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketPlaster>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::PlasterInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketPlaster>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::LanternEmissiveInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::LanternEmissiveInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::WrenInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::WrenInstanceData>", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0> >", "constructor": "rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set", "arguments": ["core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::create_buffer", "arguments": ["ref$<slice2$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<ref$<slice2$<u8> > > >", "constructor": "rhapsody::raw::device::GraphicsDevice::with_setup_cb", "arguments": ["rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<ref$<slice2$<u8> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > >", "constructor": "rhapsody::raw::device::GraphicsDevice::with_global_allocator", "arguments": ["country_core::meta_commander::commander_update::closure_env$3", "tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >", "constructor": "rhapsody::raw::device::GraphicsDevice::create_raster_pipeline", "arguments": ["alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >", "constructor": "rhapsody::raw::device::GraphicsDevice::create_compute_pipeline", "arguments": ["alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::device::Queue
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::device::Queue,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::device::Queue", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::device::Queue,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::device::Queue,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::device::Queue>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::device::Queue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::Queue::with_raw<rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0,enum2$<core::result::Result<bool,ash::vk::enums::Result> > >", "constructor": "rhapsody::raw::device::Queue::with_raw", "arguments": ["rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0", "enum2$<core::result::Result<bool,ash::vk::enums::Result> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["rhapsody::raw::device::Queue", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["rhapsody::raw::device::Queue", "alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<rhapsody::raw::device::Queue,core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["rhapsody::raw::device::Queue", "core::ops::range::Range<usize>", "rhapsody::raw::device::impl$4::create::closure_env$2", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::raw::device::Queue,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["rhapsody::raw::device::Queue", "core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::device::TimelineReclaim
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>", "constructor": "rhapsody::raw::device::TimelineReclaim", "arguments": ["rhapsody::raw::command_buffer::CommandBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >", "constructor": "slice2$", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<slice2$<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["slice2$<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::grow<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::grow", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [已有逆向证据 B / evidence_symbol_relationships; layout unknown] existing native type/interface evidence
// name: "existing native type/interface evidence"
// baseline: "B"
// status: "evidence_symbol_relationships; layout unknown"
// enum_values: "unknown"
// original_definition_file: "unknown"
// evidence: [{"path": "evidence/building-nani/B-solid-vertex-color-owner/component-query-types.json", "sha256": "bee1d0624f885d222fb60a9359ee14243ffe8c3c08eb5837c5d6e782f8f6efc8"}]
// limitations: ["Query/trait/generic symbols are type relations, not member fields", "self registration does not establish a required-component edge", "B analysis database identity may be patched; no A/C transfer"]
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: device.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "rhapsody::raw::device::GraphicsDevice", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: device.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create
// 信息 ID: e5149ee22861722e6f2bea32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::GraphicsDevicesForSurface::create_device @ 0xbdbbaf -> 0xbf2560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::debug_utils
// 信息 ID: 38c756aba59e61309fc8cc4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 16 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b9414 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b9597 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x5b9be2 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x5ba6c8 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0xfaec82 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x16a29c4 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x16a2b47 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a2e24 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a2fa7 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x16a67e2 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x16a72c8 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x17fab58 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdcac4 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdcc47 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x288e9f4 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x288eb77 -> 0xbf8960 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_mapped_buffer; module=703, inline_record=11532; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::set_buffer_debug_name; module=703, inline_record=12028; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::update_buffer_bytes; module=703, inline_record=14584; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_command_buffer; module=703, inline_record=17072; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::begin_label; module=703, inline_record=26236; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::end_label; module=703, inline_record=27276; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::set_texture_debug_name; module=703, inline_record=138704; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::update_texture_impl; module=703, inline_record=141308; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::device_uuid
// 信息 ID: a9e191af38eda36f8a266548
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_video @ 0x248280 -> 0xbf84c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info
// 信息 ID: 8766cb78764080812325f183
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> @ 0xc05190 -> 0xbf84e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::limits
// 信息 ID: 400480ea04abb5f9f034a327
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstants::new; module=705, inline_record=3700; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer; module=705, inline_record=5608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::max_bindless_descriptor_count
// 信息 ID: b7d4fefc5f58779748a51635
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set; module=703, inline_record=143740; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$0> >; module=1965, inline_record=3096; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<shader_pipeline::reflection::Binding> >,rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure_env$0> >; module=1965, inline_record=8132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::min_subgroup_size
// 信息 ID: 19d236ae22d9673977f59fdf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::taa::calculate_tgsr @ 0xa3c44f -> 0xbf8fa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::physical_device
// 信息 ID: 3278a3489ab748f1215095e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::vram::calculate_max_resolution_scale @ 0x4f7f54 -> 0xbf8950 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text @ 0x4f8279 -> 0xbf8950 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x5969e4 -> 0xbf8950 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x59d5f4 -> 0xbf8950 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui @ 0x1e1cf1a -> 0xbf8950 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::command_buffer::CommandBuffer::begin; module=702, inline_record=1936; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::device_uuid; module=703, inline_record=64324; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
// 信息 ID: cf5e0afeedafc3a79c06d386
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x4dca08 -> 0xbf8f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4ea04d -> 0xbf8f80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::rt
// 信息 ID: 637eb7410a210946d24ebd72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::validate_physical_device_requirements
// 信息 ID: eaa4bdc1801239edc8c10482
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=43832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::wait_idle
// 信息 ID: 5f8c51ff7b10921dfae86ef1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::finish_screenshot_capture @ 0x31ecd8 -> 0xbf8980 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::impl$5::drop @ 0xbe23b8 -> 0xbf8980 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::submit_pending_cbs @ 0xbec0e6 -> 0xbf8980 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::submit_pending_cbs @ 0xbec1f4 -> 0xbf8980 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources @ 0xbf8e78 -> 0xbf8980 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::immediate_empty_bucket @ 0xbf9dc0 -> 0xbf8980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
// 信息 ID: 6872d76aa829c00e5bfd98bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x4ddb94 -> 0xbf8e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e9067 -> 0xbf8e60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > >
// 信息 ID: bafc68064e8795047da29df5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::option::Option -> ["gpu_allocator::allocator::AllocatorReport"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<gpu_allocator::allocator::AllocatorReport>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "u64", "u64", "enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_global_allocator -> ["country_core::meta_commander::commander_update::closure_env$3", "tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::game_loop::shut_down_everything::closure_env$0,tuple$<u64,u64> >
// 信息 ID: 9b84243137418074fd1ff9f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["u64", "u64"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_global_allocator -> ["tiny_glade::game_loop::shut_down_everything::closure_env$0", "tuple$<u64,u64>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::run @ 0x4d898e -> 0x5b9a30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure_env$0,tuple$<> >
// 信息 ID: 3c07862702e2ae2f6d513e14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_global_allocator -> ["tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure_env$0", "tuple$<>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x14841c -> 0x5b9810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_global_allocator<toolbar::menu::report_bug::ui::closure_env$10,gpu_allocator::allocator::AllocatorReport>
// 信息 ID: 23817d32566fbe88bb1bbdb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_global_allocator -> ["toolbar::menu::report_bug::ui::closure_env$10", "gpu_allocator::allocator::AllocatorReport"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::report_bug::ui @ 0x1e1cf3e -> 0xfaeae0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > > >
// 信息 ID: acc5c33040d66c708d633976
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0 -> ["enum2$<core::option::Option<ref$<slice2$<u8> > > >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b969a -> 0xc04840 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0xc03744 -> 0xc04840 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a30aa -> 0xc04840 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdcd4a -> 0xc04840 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x288ec7a -> 0xc04840 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<ref$<slice2$<u8> > > >
// 信息 ID: b21f348745524f50a74fecef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0 -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x16a2c4a -> 0x16a3220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure_env$0>
// 信息 ID: b10cd76304209b4fd99b1544
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_buffer_bytes @ 0xbef23f -> 0xc04280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$9::init_texture_data::closure_env$1>
// 信息 ID: 5c75703bc2727683035fc38e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::texture::impl$9::init_texture_data::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::init_texture_data @ 0xbfe301 -> 0xc03fa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$9::update_texture_impl::closure_env$2>
// 信息 ID: 5d94c90de929f74595c48a61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::texture::impl$9::update_texture_impl::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_texture_impl @ 0xbfed35 -> 0xc04560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::Queue::reclaim_command_buffer_resources
// 信息 ID: 49b0fdacb11741ccf310967f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::impl$5::drop @ 0xbe23f7 -> 0xbf0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::impl$5::drop @ 0xbe2426 -> 0xbf0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources @ 0xbf8ecb -> 0xbf0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources @ 0xbf8ef6 -> 0xbf0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_submit_done @ 0xbfcb06 -> 0xbf0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_submit_done @ 0xbfcb36 -> 0xbf0cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0> @ 0xc06322 -> 0xbf0cc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::Queue::update_pending_submissions
// 信息 ID: cfa2277a771ea854072db49e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions @ 0x18c3558 -> 0xbf0b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions @ 0x18c358f -> 0xbf0b90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::Queue::with_raw
// 信息 ID: a0a5035796195cb10362bfcb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=29560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::Queue::with_raw<rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0,enum2$<core::result::Result<bool,ash::vk::enums::Result> > >
// 信息 ID: 1a5e3d0c49ad5793c71fe03e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::result::Result -> ["bool", "ash::vk::enums::Result"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::result::Result<bool,ash::vk::enums::Result>"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::Queue::with_raw -> ["rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0", "enum2$<core::result::Result<bool,ash::vk::enums::Result> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::swapchain::ManagedSwapchain::present @ 0x18cc5d2 -> 0xc03d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::create_raw_device
// 信息 ID: d6f0bfa0b515a009db26c75c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=50740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure$0
// 信息 ID: a19258dce88e24e75346ca26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::device::impl$1::reclaim_command_buffer_resources", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0 <- rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>; module=1978, inline_record=30368; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>; module=1978, inline_record=33144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$4::create::closure$0
// 信息 ID: 98485cb835d083e32ec5846d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::device::impl$4::create", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=47968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$4::create::closure$1
// 信息 ID: 0e328966d5c36bc53eed4553
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::device::impl$4::create", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=48520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$4::create::closure$2
// 信息 ID: 1fa49e23a4e3fa68df3da9a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::raw::device::impl$4::create", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::device::impl$4::create::closure_env$2 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::device::Queue,core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::device::impl$4::create::closure_env$2 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::device::Queue,core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::device::impl$4::create::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::raw::device::Queue,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::device::Queue,core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > > >; module=1935, inline_record=20996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$4::create::closure$3
// 信息 ID: c31272249b4246ca333329b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "rhapsody::raw::device::impl$4::create", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=59164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$5::drop
// 信息 ID: b737428f73839fe09be66bd4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0x5f27ed -> 0xbe23a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0x88968d -> 0xbe23a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$6::fmt
// 信息 ID: 58bed4d035e8905f9371725e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::device::DeviceCreateError> >; module=245, inline_record=400; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::device::DeviceCreateError> >; module=2310, inline_record=424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::query_device_extension_support
// 信息 ID: 88e5f14531ebac9703e3be78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=44028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::query_device_extension_support::closure$0
// 信息 ID: 634f57f0aa93750b3190d2f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::device::query_device_extension_support", "captures": "unknown"}
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xbf27b8 -> 0xbf8fb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xbf27d7 -> 0xbf8fb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xbf27f6 -> 0xbf8fb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xbf2815 -> 0xbf8fb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xbf2842 -> 0xbf8fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
