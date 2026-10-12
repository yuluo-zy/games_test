// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/save/save_thread.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 36506772cd4e460a476d2018a3655bf89c2a156af26622cc52eeff45dcd7e8fb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: save_thread.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 506, 574, 576, 577, 1585, 1592, 1613, 1629, 1654, 1686, 1736, 3208
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
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
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
// 首条位置: {"module":1686,"record_offset":46340,"rva":24255136,"va":5392964256}
// 编译实例 RVAs: 0x1721aa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-10320141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8f38662ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5392964664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10267496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8985463ff","size":5,"soft_fail":false,"target_va":5382708864,"va":5392976355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-10208350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8a23b64ff","size":5,"soft_fail":false,"target_va":5382768048,"va":5392976393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Add$LT$core..time..Duration$GT$$GT$::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10221500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8440864ff","size":5,"soft_fail":false,"target_va":5382748704,"va":5392970199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14002e900
//     owner=unresolved_namespace; count=1
//     target_aliases=130 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24095158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e84a5690fe","size":5,"soft_fail":false,"target_va":5368899840,"va":5392994993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140052a30
//     owner=unresolved_namespace; count=8
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23921776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e890fb92fe","size":5,"soft_fail":false,"target_va":5369047600,"va":5392969371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140149f50
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22930236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8c41ca2fe","size":5,"soft_fail":false,"target_va":5370060624,"va":5392990855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22841286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e83a78a3fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5392999409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ca2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21351398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e81a34bafe","size":5,"soft_fail":false,"target_va":5371634416,"va":5392985809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5d80
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21233300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e86c01bcfe","size":5,"soft_fail":false,"target_va":5371747712,"va":5392981007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403107e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21050406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8dacbbefe","size":5,"soft_fail":false,"target_va":5371922400,"va":5392972801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19338854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e89ae9d8fe","size":5,"soft_fail":false,"target_va":5373629488,"va":5392968337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d8fc0
//     owner=unresolved_namespace; count=4
//     target_aliases=23 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18150400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8000cebfe","size":5,"soft_fail":false,"target_va":5374840768,"va":5392991163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=8
//     target_aliases=92 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18076858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8462becfe","size":5,"soft_fail":false,"target_va":5374900736,"va":5392977589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18090863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e891f4ebfe","size":5,"soft_fail":false,"target_va":5374904480,"va":5392995338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ecc10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18059793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8ef6decfe","size":5,"soft_fail":false,"target_va":5374921744,"va":5392981532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17897506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8dee7eefe","size":5,"soft_fail":false,"target_va":5375081488,"va":5392978989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=97
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17886241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8df13effe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392967740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=14
//     target_aliases=69 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17448448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e800c2f5fe","size":5,"soft_fail":false,"target_va":5375516032,"va":5392964475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140876ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15409535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e881de14ff","size":5,"soft_fail":false,"target_va":5377584288,"va":5392993818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140877160
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15397572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e83c0d15ff","size":5,"soft_fail":false,"target_va":5377585504,"va":5392983071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408778c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15403304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8d8f614ff","size":5,"soft_fail":false,"target_va":5377587392,"va":5392990691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140877b30
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15383105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8bf4515ff","size":5,"soft_fail":false,"target_va":5377588016,"va":5392971116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140878420
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15379041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e89f5515ff","size":5,"soft_fail":false,"target_va":5377590304,"va":5392969340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087cfb0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15359685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e83ba115ff","size":5,"soft_fail":false,"target_va":5377609648,"va":5392969328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408800b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15367766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8aa8115ff","size":5,"soft_fail":false,"target_va":5377622192,"va":5392989953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408802e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15354854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e81ab415ff","size":5,"soft_fail":false,"target_va":5377622752,"va":5392977601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088bae0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15314568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8785116ff","size":5,"soft_fail":false,"target_va":5377669856,"va":5392984419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088eb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15297660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8849316ff","size":5,"soft_fail":false,"target_va":5377682240,"va":5392979895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088ed10
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15286379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e895bf16ff","size":5,"soft_fail":false,"target_va":5377682704,"va":5392969078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140890ff0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15295828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8ac9a16ff","size":5,"soft_fail":false,"target_va":5377691632,"va":5392987455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140894d60
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15287301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8fbbb16ff","size":5,"soft_fail":false,"target_va":5377707360,"va":5392994656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408da1b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14979874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8de6c1bff","size":5,"soft_fail":false,"target_va":5377991088,"va":5392970957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140914ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14740550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8ba131fff","size":5,"soft_fail":false,"target_va":5378231008,"va":5392971553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14091a450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14730817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8bf391fff","size":5,"soft_fail":false,"target_va":5378253904,"va":5392984716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14396867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e83d5224ff","size":5,"soft_fail":false,"target_va":5378584624,"va":5392981486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b1c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14395384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8085824ff","size":5,"soft_fail":false,"target_va":5378585024,"va":5392980403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14400136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8784524ff","size":5,"soft_fail":false,"target_va":5378585200,"va":5392985331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6e0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13333755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8058b34ff","size":5,"soft_fail":false,"target_va":5379645616,"va":5392979366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0cca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12684441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e867733eff","size":5,"soft_fail":false,"target_va":5380295840,"va":5392980276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b4c000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12432504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8884b42ff","size":5,"soft_fail":false,"target_va":5380554752,"va":5392987251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10272502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e80a4163ff","size":5,"soft_fail":false,"target_va":5382704752,"va":5392977249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=24
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10139950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8d24665ff","size":5,"soft_fail":false,"target_va":5382827728,"va":5392967673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fe41e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7614290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8aed08bff","size":5,"soft_fail":false,"target_va":5385372128,"va":5392986413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e6780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4446716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e80426bcff","size":5,"soft_fail":false,"target_va":5388527488,"va":5392974199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f81e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4385948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e86413bdff","size":5,"soft_fail":false,"target_va":5388599776,"va":5392985719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14130b810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4302902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8ca57beff","size":5,"soft_fail":false,"target_va":5388679184,"va":5392982081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14145b1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2920161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e81f71d3ff","size":5,"soft_fail":false,"target_va":5390053792,"va":5392973948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141527a70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2088745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8d720e0ff","size":5,"soft_fail":false,"target_va":5390891632,"va":5392980372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a4f40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1563302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e85a25e8ff","size":5,"soft_fail":false,"target_va":5391404864,"va":5392968161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a5fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1564370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e82e21e8ff","size":5,"soft_fail":false,"target_va":5391409120,"va":5392973485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f6360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-186036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e84c29fdff","size":5,"soft_fail":false,"target_va":5392786272,"va":5392972303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141750c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t179414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8d6bc0200","size":5,"soft_fail":false,"target_va":5393157264,"va":5392977845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18321290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e88a8f1701","size":5,"soft_fail":false,"target_va":5411285568,"va":5392964273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18369373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e85d4b1801","size":5,"soft_fail":false,"target_va":5411340320,"va":5392970942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18356161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8c1171801","size":5,"soft_fail":false,"target_va":5411342880,"va":5392986714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8e80
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18375475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e833631801","size":5,"soft_fail":false,"target_va":5411344000,"va":5392968520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9590
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18365366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8b63b1801","size":5,"soft_fail":false,"target_va":5411345808,"va":5392980437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18360478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e89e281801","size":5,"soft_fail":false,"target_va":5411346048,"va":5392985565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428aa7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18364647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8e7381801","size":5,"soft_fail":false,"target_va":5411350432,"va":5392985780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428b2e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18399216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8f0bf1801","size":5,"soft_fail":false,"target_va":5411384896,"va":5392985675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=19
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18700447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e89f581d01","size":5,"soft_fail":false,"target_va":5411669968,"va":5392969516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18874788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8a4012001","size":5,"soft_fail":false,"target_va":5411847152,"va":5392972359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=21
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18878120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8a80e2001","size":5,"soft_fail":false,"target_va":5411847168,"va":5392969043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18862528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8c0d11f01","size":5,"soft_fail":false,"target_va":5411847200,"va":5392984667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18554170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e83a1d1b01","size":5,"soft_fail":false,"target_va":5411540992,"va":5392986817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-10127705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8a77665ff","size":5,"soft_fail":false,"target_va":5382840208,"va":5392967908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18620918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e80adee3fe","size":5,"soft_fail":false,"target_va":5374356528,"va":5392977441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18360792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8d8291801","size":5,"soft_fail":false,"target_va":5411347040,"va":5392986243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18554114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8021d1b01","size":5,"soft_fail":false,"target_va":5411540963,"va":5392986844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=14
//     evidence={"assembly":"\tcallq\t-10292839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e899f162ff","size":5,"soft_fail":false,"target_va":5382683584,"va":5392976418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10313814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8aa9f62ff","size":5,"soft_fail":false,"target_va":5382655024,"va":5392968833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=16
//     evidence={"assembly":"\tcallq\t18555965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e83d241b01","size":5,"soft_fail":false,"target_va":5411541584,"va":5392985614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18555341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8cd211b01","size":5,"soft_fail":false,"target_va":5411541776,"va":5392986430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18556414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8fe251b01","size":5,"soft_fail":false,"target_va":5411542064,"va":5392985645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18555399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e807221b01","size":5,"soft_fail":false,"target_va":5411541904,"va":5392986500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<alloc::boxed::Box<country_core::systems::save::save_game::GameSaveData,alloc::alloc::Global> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2861446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e87a56d4ff","size":5,"soft_fail":false,"target_va":5390125888,"va":5392987329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<alloc::boxed::Box<country_core::systems::save::snapshot::SnapshotSaveData,alloc::alloc::Global> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13551193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8a73931ff","size":5,"soft_fail":false,"target_va":5379450784,"va":5393001972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> > > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-38154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8f66affff","size":5,"soft_fail":false,"target_va":5392950016,"va":5392988165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::ReadDir,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-51807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8a135ffff","size":5,"soft_fail":false,"target_va":5392950928,"va":5393002730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18556668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8fc261b01","size":5,"soft_fail":false,"target_va":5411542880,"va":5392986207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-18646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e82ab7ffff","size":5,"soft_fail":false,"target_va":5392951088,"va":5392969729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t18576304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8b0731b01","size":5,"soft_fail":false,"target_va":5411543360,"va":5392967051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::time::Duration::from_secs_f32
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e885b9ffff","size":5,"soft_fail":false,"target_va":5392952144,"va":5392970182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::backup_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12437091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e89d3942ff","size":5,"soft_fail":false,"target_va":5380530560,"va":5392967646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12437490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e80e3842ff","size":5,"soft_fail":false,"target_va":5380530176,"va":5392967661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_game::add_history_edit_to_save_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13512174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e812d231ff","size":5,"soft_fail":false,"target_va":5379471792,"va":5392983961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::CurrentSaveFile::change_section
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e87cbaffff","size":5,"soft_fail":false,"target_va":5392952544,"va":5392970335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e89dceffff","size":5,"soft_fail":false,"target_va":5392955376,"va":5392968014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::UnsavedFileList::add
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e87ad1ffff","size":5,"soft_fail":false,"target_va":5392957312,"va":5392969217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::shadow_file::ShadowFile::create
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14732225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e83f341fff","size":5,"soft_fail":false,"target_va":5378245840,"va":5392978060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::shadow_file::ShadowFile::flush
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14720672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e860611fff","size":5,"soft_fail":false,"target_va":5378249680,"va":5392970347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::snapshot::capture_game_snapshot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t515272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8c8dc0700","size":5,"soft_fail":false,"target_va":5393489152,"va":5392973875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18506470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8e6621a01","size":5,"soft_fail":false,"target_va":5411492368,"va":5392985893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18508465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8b16a1a01","size":5,"soft_fail":false,"target_va":5411494800,"va":5392986330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11148137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e897e455ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5392971732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11148671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e881e255ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5392972682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11582054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e89a454fff","size":5,"soft_fail":false,"target_va":5381385536,"va":5392967585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=19
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11576858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8e6594fff","size":5,"soft_fail":false,"target_va":5381390640,"va":5392967493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11582974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e802424fff","size":5,"soft_fail":false,"target_va":5381386480,"va":5392969449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11579974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8ba4d4fff","size":5,"soft_fail":false,"target_va":5381384704,"va":5392964673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-10273769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8173c63ff","size":5,"soft_fail":false,"target_va":5382694656,"va":5392968420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10264747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8555f63ff","size":5,"soft_fail":false,"target_va":5382709584,"va":5392974326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10268184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8e85163ff","size":5,"soft_fail":false,"target_va":5382709008,"va":5392977187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-10262163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e86d6963ff","size":5,"soft_fail":false,"target_va":5382706224,"va":5392968382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10320651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8f58462ff","size":5,"soft_fail":false,"target_va":5382657856,"va":5392978502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-10223632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8f0ff63ff","size":5,"soft_fail":false,"target_va":5382744080,"va":5392967707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::components
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10226772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8acf363ff","size":5,"soft_fail":false,"target_va":5382745088,"va":5392971855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10233260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e854da63ff","size":5,"soft_fail":false,"target_va":5382743968,"va":5392977223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10240845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8b3bc63ff","size":5,"soft_fail":false,"target_va":5382741504,"va":5392982344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_file
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10228940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e834eb63ff","size":5,"soft_fail":false,"target_va":5382745328,"va":5392974263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10189113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8c78664ff","size":5,"soft_fail":false,"target_va":5382778992,"va":5392968100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10204825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8674964ff","size":5,"soft_fail":false,"target_va":5382770560,"va":5392975380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::unlink
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10210481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e84f3364ff","size":5,"soft_fail":false,"target_va":5382772112,"va":5392982588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10224752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e890fb63ff","size":5,"soft_fail":false,"target_va":5382753792,"va":5392978539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10288166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8da0363ff","size":5,"soft_fail":false,"target_va":5382691056,"va":5392979217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t-4145324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e854bfc0ff","size":5,"soft_fail":false,"target_va":5388825296,"va":5392970615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=16
//     evidence={"assembly":"\tcallq\t18551857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e831141b01","size":5,"soft_fail":false,"target_va":5411537104,"va":5392985242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-10222552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392964256,"raw_hex":"e8280464ff","size":5,"soft_fail":false,"target_va":5382747616,"va":5392970163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392964256,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5392970031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":92908,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":46688,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1686,"record_offset":84800,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1686,"record_offset":68156,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":55140,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":69220,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":68184,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":81108,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1686,"record_offset":90812,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":90896,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MaybePendingSnapshotId::assign
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1686,"record_offset":129244,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":31751,"source_line":183,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MaybePendingSnapshotId::assign_invalid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1686,"record_offset":124028,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":31752,"source_line":187,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$22::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1686,"record_offset":87120,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":76809,"source_line":123,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_game::SaveFileSection::save_cooldown_seconds
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_game.rs
//     evidence={"module":1686,"record_offset":81152,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":31323,"source_line":510,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::CurrentSaveFile::backup_save
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":56540,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":76806,"source_line":187,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::CurrentSaveFile::perform_manual_backup
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":87232,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":76810,"source_line":266,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::CurrentSaveFile::replace_edits
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":72148,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":76807,"source_line":65,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::capture_snapshot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":96316,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":76814,"source_line":793,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::execute_open_file_cmd
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":102452,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":76815,"source_line":670,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":92028,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":28712,"source_line":112,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":90956,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":28716,"source_line":247,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1686,"record_offset":90656,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":46720,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":46604,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":55528,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":46896,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":92056,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":90312,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$131::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":91124,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":76812,"source_line":3400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":90172,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::SyncSender::send
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":1686,"record_offset":84828,"parent_record_offset":46340,"ancestor_record_offset":46340,"inlinee":7840,"source_line":673,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":118552,"parent_record_offset":118528,"ancestor_record_offset":46340,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":118576,"parent_record_offset":118552,"ancestor_record_offset":46340,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=97
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":57224,"parent_record_offset":57200,"ancestor_record_offset":46340,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":118496,"parent_record_offset":118460,"ancestor_record_offset":46340,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":118528,"parent_record_offset":118496,"ancestor_record_offset":46340,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=97
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":57200,"parent_record_offset":57144,"ancestor_record_offset":46340,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1686,"record_offset":118460,"parent_record_offset":118424,"ancestor_record_offset":46340,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1686,"record_offset":64056,"parent_record_offset":64028,"ancestor_record_offset":46340,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1686,"record_offset":57428,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1686,"record_offset":57476,"parent_record_offset":57452,"ancestor_record_offset":46340,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":77532,"parent_record_offset":77504,"ancestor_record_offset":46340,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":62340,"parent_record_offset":62300,"ancestor_record_offset":46340,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":77668,"parent_record_offset":77636,"ancestor_record_offset":46340,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":118960,"parent_record_offset":118924,"ancestor_record_offset":46340,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":77560,"parent_record_offset":77532,"ancestor_record_offset":46340,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":57172,"parent_record_offset":57144,"ancestor_record_offset":46340,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=70
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":57144,"parent_record_offset":57120,"ancestor_record_offset":46340,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":77732,"parent_record_offset":77700,"ancestor_record_offset":46340,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":62396,"parent_record_offset":62368,"ancestor_record_offset":46340,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":62368,"parent_record_offset":62340,"ancestor_record_offset":46340,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":77700,"parent_record_offset":77668,"ancestor_record_offset":46340,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":119036,"parent_record_offset":118996,"ancestor_record_offset":46340,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":118996,"parent_record_offset":118960,"ancestor_record_offset":46340,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=70
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":57120,"parent_record_offset":57096,"ancestor_record_offset":46340,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":118884,"parent_record_offset":118848,"ancestor_record_offset":46340,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":118848,"parent_record_offset":118812,"ancestor_record_offset":46340,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":62080,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":118776,"parent_record_offset":118728,"ancestor_record_offset":46340,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":118812,"parent_record_offset":118776,"ancestor_record_offset":46340,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":118728,"parent_record_offset":118692,"ancestor_record_offset":46340,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":62116,"parent_record_offset":62080,"ancestor_record_offset":46340,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1686,"record_offset":118692,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1686,"record_offset":108056,"parent_record_offset":108020,"ancestor_record_offset":46340,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1686,"record_offset":108020,"parent_record_offset":107996,"ancestor_record_offset":46340,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":112248,"parent_record_offset":112200,"ancestor_record_offset":46340,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":62300,"parent_record_offset":62272,"ancestor_record_offset":46340,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":77504,"parent_record_offset":77400,"ancestor_record_offset":46340,"inlinee":7415,"source_line":1246,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":77600,"parent_record_offset":77400,"ancestor_record_offset":46340,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":103852,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":62272,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":112200,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":77636,"parent_record_offset":77400,"ancestor_record_offset":46340,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":77936,"parent_record_offset":77400,"ancestor_record_offset":46340,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":77808,"parent_record_offset":77400,"ancestor_record_offset":46340,"inlinee":8644,"source_line":2876,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":75312,"parent_record_offset":72148,"ancestor_record_offset":46340,"inlinee":4915,"source_line":1524,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":118924,"parent_record_offset":118884,"ancestor_record_offset":46340,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":62044,"parent_record_offset":62016,"ancestor_record_offset":46340,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":75344,"parent_record_offset":72148,"ancestor_record_offset":46340,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":76344,"parent_record_offset":76304,"ancestor_record_offset":46340,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":76304,"parent_record_offset":76272,"ancestor_record_offset":46340,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":76628,"parent_record_offset":76604,"ancestor_record_offset":46340,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":75408,"parent_record_offset":72148,"ancestor_record_offset":46340,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":76428,"parent_record_offset":76400,"ancestor_record_offset":46340,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1686,"record_offset":113596,"parent_record_offset":113568,"ancestor_record_offset":46340,"inlinee":12447,"source_line":316,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1686,"record_offset":59632,"parent_record_offset":59588,"ancestor_record_offset":46340,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":59588,"parent_record_offset":59552,"ancestor_record_offset":46340,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":113568,"parent_record_offset":113532,"ancestor_record_offset":46340,"inlinee":12446,"source_line":33,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":59552,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":1686,"record_offset":78428,"parent_record_offset":78400,"ancestor_record_offset":46340,"inlinee":11197,"source_line":333,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":1686,"record_offset":78492,"parent_record_offset":78464,"ancestor_record_offset":46340,"inlinee":25488,"source_line":350,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":1686,"record_offset":78400,"parent_record_offset":78368,"ancestor_record_offset":46340,"inlinee":25242,"source_line":774,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::set_len_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec.rs
//     evidence={"module":1686,"record_offset":78464,"parent_record_offset":78368,"ancestor_record_offset":46340,"inlinee":25487,"source_line":585,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":1686,"record_offset":78368,"parent_record_offset":78148,"ancestor_record_offset":46340,"inlinee":28645,"source_line":359,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1686,"record_offset":64088,"parent_record_offset":64056,"ancestor_record_offset":46340,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1686,"record_offset":87168,"parent_record_offset":87144,"ancestor_record_offset":46340,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1686,"record_offset":87192,"parent_record_offset":87168,"ancestor_record_offset":46340,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":55868,"parent_record_offset":55840,"ancestor_record_offset":46340,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":116016,"parent_record_offset":115984,"ancestor_record_offset":46340,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=56
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":55920,"parent_record_offset":55896,"ancestor_record_offset":46340,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":55896,"parent_record_offset":55868,"ancestor_record_offset":46340,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":55816,"parent_record_offset":55788,"ancestor_record_offset":46340,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":55840,"parent_record_offset":55816,"ancestor_record_offset":46340,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":56012,"parent_record_offset":55840,"ancestor_record_offset":46340,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":56124,"parent_record_offset":56100,"ancestor_record_offset":46340,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":111448,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":59240,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":91932,"parent_record_offset":91300,"ancestor_record_offset":46340,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1686,"record_offset":58256,"parent_record_offset":58052,"ancestor_record_offset":46340,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1686,"record_offset":59272,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1686,"record_offset":61928,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1686,"record_offset":57512,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":78028,"parent_record_offset":78000,"ancestor_record_offset":46340,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":119192,"parent_record_offset":119168,"ancestor_record_offset":46340,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":62196,"parent_record_offset":62152,"ancestor_record_offset":46340,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":75232,"parent_record_offset":75140,"ancestor_record_offset":46340,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":1686,"record_offset":116452,"parent_record_offset":116424,"ancestor_record_offset":46340,"inlinee":9901,"source_line":57,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::Flatten::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1686,"record_offset":106344,"parent_record_offset":106316,"ancestor_record_offset":46340,"inlinee":27670,"source_line":244,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1686,"record_offset":107752,"parent_record_offset":107672,"ancestor_record_offset":46340,"inlinee":11087,"source_line":807,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$24::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1686,"record_offset":106412,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":11083,"source_line":280,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1686,"record_offset":107672,"parent_record_offset":107636,"ancestor_record_offset":46340,"inlinee":11085,"source_line":573,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$59::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1686,"record_offset":106456,"parent_record_offset":106412,"ancestor_record_offset":46340,"inlinee":13694,"source_line":906,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1686,"record_offset":107548,"parent_record_offset":107500,"ancestor_record_offset":46340,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1686,"record_offset":107500,"parent_record_offset":106456,"ancestor_record_offset":46340,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1686,"record_offset":116424,"parent_record_offset":116396,"ancestor_record_offset":46340,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1686,"record_offset":116572,"parent_record_offset":116508,"ancestor_record_offset":46340,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1686,"record_offset":91520,"parent_record_offset":91360,"ancestor_record_offset":46340,"inlinee":10401,"source_line":20,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1686,"record_offset":106380,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1686,"record_offset":76400,"parent_record_offset":76372,"ancestor_record_offset":46340,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1686,"record_offset":76372,"parent_record_offset":76344,"ancestor_record_offset":46340,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1686,"record_offset":62016,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::flatten
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1686,"record_offset":106316,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":27668,"source_line":1514,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1686,"record_offset":116396,"parent_record_offset":116268,"ancestor_record_offset":46340,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1686,"record_offset":116768,"parent_record_offset":116744,"ancestor_record_offset":46340,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=62
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":55944,"parent_record_offset":55920,"ancestor_record_offset":46340,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":64116,"parent_record_offset":64088,"ancestor_record_offset":46340,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":78204,"parent_record_offset":78148,"ancestor_record_offset":46340,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":69644,"parent_record_offset":69616,"ancestor_record_offset":46340,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":69672,"parent_record_offset":69644,"ancestor_record_offset":46340,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":77472,"parent_record_offset":77400,"ancestor_record_offset":46340,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":116836,"parent_record_offset":116508,"ancestor_record_offset":46340,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":69956,"parent_record_offset":69924,"ancestor_record_offset":46340,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":77760,"parent_record_offset":77732,"ancestor_record_offset":46340,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1686,"record_offset":47756,"parent_record_offset":47724,"ancestor_record_offset":46340,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":75140,"parent_record_offset":75112,"ancestor_record_offset":46340,"inlinee":14148,"source_line":593,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":78332,"parent_record_offset":78148,"ancestor_record_offset":46340,"inlinee":28676,"source_line":3197,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":75196,"parent_record_offset":75168,"ancestor_record_offset":46340,"inlinee":14150,"source_line":2350,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":75168,"parent_record_offset":75140,"ancestor_record_offset":46340,"inlinee":14149,"source_line":2431,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":116044,"parent_record_offset":115984,"ancestor_record_offset":46340,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1686,"record_offset":116596,"parent_record_offset":116572,"ancestor_record_offset":46340,"inlinee":4196,"source_line":166,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1686,"record_offset":55680,"parent_record_offset":55636,"ancestor_record_offset":46340,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":111360,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":107596,"parent_record_offset":107548,"ancestor_record_offset":46340,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":91728,"parent_record_offset":91696,"ancestor_record_offset":46340,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":75036,"parent_record_offset":72148,"ancestor_record_offset":46340,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":57452,"parent_record_offset":57428,"ancestor_record_offset":46340,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":102732,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":62444,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":115736,"parent_record_offset":115708,"ancestor_record_offset":46340,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":111472,"parent_record_offset":111448,"ancestor_record_offset":46340,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":110328,"parent_record_offset":110252,"ancestor_record_offset":46340,"inlinee":6199,"source_line":2499,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":91424,"parent_record_offset":91360,"ancestor_record_offset":46340,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":46776,"parent_record_offset":46744,"ancestor_record_offset":46340,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":46804,"parent_record_offset":46744,"ancestor_record_offset":46340,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":47840,"parent_record_offset":47812,"ancestor_record_offset":46340,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":56492,"parent_record_offset":55636,"ancestor_record_offset":46340,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":119168,"parent_record_offset":118884,"ancestor_record_offset":46340,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":76512,"parent_record_offset":76484,"ancestor_record_offset":46340,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=580
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":56076,"parent_record_offset":55788,"ancestor_record_offset":46340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":75372,"parent_record_offset":75344,"ancestor_record_offset":46340,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":78000,"parent_record_offset":77972,"ancestor_record_offset":46340,"inlinee":9440,"source_line":1455,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":115824,"parent_record_offset":115708,"ancestor_record_offset":46340,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":69708,"parent_record_offset":69556,"ancestor_record_offset":46340,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":76484,"parent_record_offset":76456,"ancestor_record_offset":46340,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":63836,"parent_record_offset":63804,"ancestor_record_offset":46340,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":63892,"parent_record_offset":63864,"ancestor_record_offset":46340,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":63864,"parent_record_offset":63836,"ancestor_record_offset":46340,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":75496,"parent_record_offset":75408,"ancestor_record_offset":46340,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":108088,"parent_record_offset":108056,"ancestor_record_offset":46340,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":47320,"parent_record_offset":47296,"ancestor_record_offset":46340,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":75528,"parent_record_offset":75408,"ancestor_record_offset":46340,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":47296,"parent_record_offset":47272,"ancestor_record_offset":46340,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":76456,"parent_record_offset":76428,"ancestor_record_offset":46340,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":47504,"parent_record_offset":47476,"ancestor_record_offset":46340,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":69280,"parent_record_offset":69248,"ancestor_record_offset":46340,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":118604,"parent_record_offset":118576,"ancestor_record_offset":46340,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":115848,"parent_record_offset":115824,"ancestor_record_offset":46340,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=62
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":55968,"parent_record_offset":55944,"ancestor_record_offset":46340,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":121116,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::inspect_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":104980,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":13626,"source_line":899,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":61628,"parent_record_offset":61592,"ancestor_record_offset":46340,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":105396,"parent_record_offset":105356,"ancestor_record_offset":46340,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":75272,"parent_record_offset":75072,"ancestor_record_offset":46340,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":109588,"parent_record_offset":109536,"ancestor_record_offset":46340,"inlinee":32593,"source_line":1908,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":96432,"parent_record_offset":96316,"ancestor_record_offset":46340,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":59128,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$6::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":105356,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":32591,"source_line":1768,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":91956,"parent_record_offset":91932,"ancestor_record_offset":46340,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":91980,"parent_record_offset":91956,"ancestor_record_offset":46340,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1686,"record_offset":77872,"parent_record_offset":77400,"ancestor_record_offset":46340,"inlinee":8645,"source_line":3567,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1686,"record_offset":113420,"parent_record_offset":113284,"ancestor_record_offset":46340,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1686,"record_offset":90836,"parent_record_offset":90812,"ancestor_record_offset":46340,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":47248,"parent_record_offset":47224,"ancestor_record_offset":46340,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":47272,"parent_record_offset":47248,"ancestor_record_offset":46340,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":47576,"parent_record_offset":47548,"ancestor_record_offset":46340,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":116508,"parent_record_offset":116480,"ancestor_record_offset":46340,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":47224,"parent_record_offset":47108,"ancestor_record_offset":46340,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1686,"record_offset":62152,"parent_record_offset":62116,"ancestor_record_offset":46340,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\specialize.rs
//     evidence={"module":1686,"record_offset":77900,"parent_record_offset":77872,"ancestor_record_offset":46340,"inlinee":8647,"source_line":18,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1686,"record_offset":47724,"parent_record_offset":47692,"ancestor_record_offset":46340,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1686,"record_offset":46932,"parent_record_offset":46896,"ancestor_record_offset":46340,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1686,"record_offset":47076,"parent_record_offset":47044,"ancestor_record_offset":46340,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1686,"record_offset":47044,"parent_record_offset":47004,"ancestor_record_offset":46340,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":46980,"parent_record_offset":46956,"ancestor_record_offset":46340,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":47004,"parent_record_offset":46980,"ancestor_record_offset":46340,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":46956,"parent_record_offset":46932,"ancestor_record_offset":46340,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":47548,"parent_record_offset":47004,"ancestor_record_offset":46340,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":72456,"parent_record_offset":72420,"ancestor_record_offset":46340,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":72420,"parent_record_offset":72392,"ancestor_record_offset":46340,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":47692,"parent_record_offset":47656,"ancestor_record_offset":46340,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":47788,"parent_record_offset":47692,"ancestor_record_offset":46340,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":47656,"parent_record_offset":47628,"ancestor_record_offset":46340,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":111520,"parent_record_offset":111496,"ancestor_record_offset":46340,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":47628,"parent_record_offset":46896,"ancestor_record_offset":46340,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":47812,"parent_record_offset":47788,"ancestor_record_offset":46340,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":47108,"parent_record_offset":47076,"ancestor_record_offset":46340,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":47392,"parent_record_offset":47108,"ancestor_record_offset":46340,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":47364,"parent_record_offset":47108,"ancestor_record_offset":46340,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":47168,"parent_record_offset":47108,"ancestor_record_offset":46340,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":124056,"parent_record_offset":124028,"ancestor_record_offset":46340,"inlinee":19451,"source_line":2725,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":93148,"parent_record_offset":93084,"ancestor_record_offset":46340,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":90720,"parent_record_offset":90684,"ancestor_record_offset":46340,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":108568,"parent_record_offset":108532,"ancestor_record_offset":46340,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":46628,"parent_record_offset":46604,"ancestor_record_offset":46340,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":93172,"parent_record_offset":93148,"ancestor_record_offset":46340,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":90744,"parent_record_offset":90720,"ancestor_record_offset":46340,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":46652,"parent_record_offset":46628,"ancestor_record_offset":46340,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":124084,"parent_record_offset":124056,"ancestor_record_offset":46340,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":108116,"parent_record_offset":108056,"ancestor_record_offset":46340,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":108148,"parent_record_offset":108020,"ancestor_record_offset":46340,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::HistorySnapshotId::file_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1686,"record_offset":96664,"parent_record_offset":96316,"ancestor_record_offset":46340,"inlinee":31761,"source_line":283,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::HistorySnapshotId::from_snapshot_blob
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1686,"record_offset":96464,"parent_record_offset":96316,"ancestor_record_offset":46340,"inlinee":31760,"source_line":279,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_backup_list::SaveBackupList::into_files
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":1686,"record_offset":115396,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":32631,"source_line":136,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":1686,"record_offset":61548,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":32582,"source_line":54,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::execute_open_file_cmd::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":105032,"parent_record_offset":104980,"ancestor_record_offset":46340,"inlinee":76816,"source_line":692,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::full_write_history_file
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":113284,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":76817,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFile::data
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":59300,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":28712,"source_line":112,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFile::truncate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":78148,"parent_record_offset":72148,"ancestor_record_offset":46340,"inlinee":28711,"source_line":103,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFile::write_at
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":75072,"parent_record_offset":72148,"ancestor_record_offset":46340,"inlinee":28710,"source_line":93,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":77244,"parent_record_offset":77204,"ancestor_record_offset":46340,"inlinee":28698,"source_line":289,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFileWriter::write_all
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":77204,"parent_record_offset":72148,"ancestor_record_offset":46340,"inlinee":28721,"source_line":281,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":116300,"parent_record_offset":116268,"ancestor_record_offset":46340,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":116268,"parent_record_offset":116240,"ancestor_record_offset":46340,"inlinee":6915,"source_line":4480,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":116792,"parent_record_offset":116768,"ancestor_record_offset":46340,"inlinee":6918,"source_line":4492,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":116328,"parent_record_offset":116300,"ancestor_record_offset":46340,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":69556,"parent_record_offset":69524,"ancestor_record_offset":46340,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":69588,"parent_record_offset":69556,"ancestor_record_offset":46340,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":69468,"parent_record_offset":69444,"ancestor_record_offset":46340,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":69524,"parent_record_offset":69468,"ancestor_record_offset":46340,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":69496,"parent_record_offset":69468,"ancestor_record_offset":46340,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":69616,"parent_record_offset":69556,"ancestor_record_offset":46340,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":69444,"parent_record_offset":69420,"ancestor_record_offset":46340,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1686,"record_offset":61024,"parent_record_offset":61000,"ancestor_record_offset":46340,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1686,"record_offset":116240,"parent_record_offset":116212,"ancestor_record_offset":46340,"inlinee":6932,"source_line":1307,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1686,"record_offset":93056,"parent_record_offset":93028,"ancestor_record_offset":46340,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":46832,"parent_record_offset":46744,"ancestor_record_offset":46340,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":62776,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":46744,"parent_record_offset":46720,"ancestor_record_offset":46340,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":46856,"parent_record_offset":46832,"ancestor_record_offset":46340,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":59156,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1686,"record_offset":90684,"parent_record_offset":90656,"ancestor_record_offset":46340,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1686,"record_offset":93084,"parent_record_offset":93056,"ancestor_record_offset":46340,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1686,"record_offset":90776,"parent_record_offset":90684,"ancestor_record_offset":46340,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1686,"record_offset":93120,"parent_record_offset":93084,"ancestor_record_offset":46340,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":73932,"parent_record_offset":72148,"ancestor_record_offset":46340,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":55564,"parent_record_offset":55528,"ancestor_record_offset":46340,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":72348,"parent_record_offset":72148,"ancestor_record_offset":46340,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":70272,"parent_record_offset":70240,"ancestor_record_offset":46340,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":70788,"parent_record_offset":70528,"ancestor_record_offset":46340,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=19
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1686,"record_offset":55708,"parent_record_offset":55680,"ancestor_record_offset":46340,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":55788,"parent_record_offset":55636,"ancestor_record_offset":46340,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":56048,"parent_record_offset":55788,"ancestor_record_offset":46340,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":72844,"parent_record_offset":72148,"ancestor_record_offset":46340,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":61000,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":116184,"parent_record_offset":115464,"ancestor_record_offset":46340,"inlinee":13714,"source_line":392,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$7::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":115464,"parent_record_offset":115424,"ancestor_record_offset":46340,"inlinee":13713,"source_line":1067,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":116212,"parent_record_offset":115464,"ancestor_record_offset":46340,"inlinee":13716,"source_line":1117,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":111412,"parent_record_offset":111388,"ancestor_record_offset":46340,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":116648,"parent_record_offset":116620,"ancestor_record_offset":46340,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":56852,"parent_record_offset":56828,"ancestor_record_offset":46340,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":56900,"parent_record_offset":56876,"ancestor_record_offset":46340,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":56876,"parent_record_offset":56852,"ancestor_record_offset":46340,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":97420,"parent_record_offset":97172,"ancestor_record_offset":46340,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":97196,"parent_record_offset":97172,"ancestor_record_offset":46340,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":58052,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":12455,"source_line":2523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":97172,"parent_record_offset":96316,"ancestor_record_offset":46340,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":61592,"parent_record_offset":61548,"ancestor_record_offset":46340,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":62968,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":8068,"source_line":2342,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":59328,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1686,"record_offset":115536,"parent_record_offset":115512,"ancestor_record_offset":46340,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1686,"record_offset":115784,"parent_record_offset":115760,"ancestor_record_offset":46340,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1686,"record_offset":115648,"parent_record_offset":115620,"ancestor_record_offset":46340,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1686,"record_offset":115984,"parent_record_offset":115584,"ancestor_record_offset":46340,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1686,"record_offset":115512,"parent_record_offset":115464,"ancestor_record_offset":46340,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Write::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":1686,"record_offset":77276,"parent_record_offset":77204,"ancestor_record_offset":46340,"inlinee":8631,"source_line":1754,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::into_inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":113644,"parent_record_offset":113284,"ancestor_record_offset":46340,"inlinee":76818,"source_line":116,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":113392,"parent_record_offset":113284,"ancestor_record_offset":46340,"inlinee":21628,"source_line":98,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":75112,"parent_record_offset":75072,"ancestor_record_offset":46340,"inlinee":14156,"source_line":293,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$9::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":77308,"parent_record_offset":77276,"ancestor_record_offset":46340,"inlinee":28700,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":77400,"parent_record_offset":77340,"ancestor_record_offset":46340,"inlinee":8643,"source_line":443,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":77340,"parent_record_offset":77308,"ancestor_record_offset":46340,"inlinee":8642,"source_line":504,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":77972,"parent_record_offset":77340,"ancestor_record_offset":46340,"inlinee":8648,"source_line":486,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":63804,"parent_record_offset":63772,"ancestor_record_offset":46340,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":63772,"parent_record_offset":63740,"ancestor_record_offset":46340,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":1686,"record_offset":98976,"parent_record_offset":98952,"ancestor_record_offset":46340,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":91696,"parent_record_offset":91300,"ancestor_record_offset":46340,"inlinee":10391,"source_line":646,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":56944,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":56828,"parent_record_offset":56804,"ancestor_record_offset":46340,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Prefix::is_verbatim
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":91792,"parent_record_offset":91764,"ancestor_record_offset":46340,"inlinee":10393,"source_line":222,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$100::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":91668,"parent_record_offset":91300,"ancestor_record_offset":46340,"inlinee":10389,"source_line":371,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$116::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":91360,"parent_record_offset":91300,"ancestor_record_offset":46340,"inlinee":10395,"source_line":589,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":91300,"parent_record_offset":91152,"ancestor_record_offset":46340,"inlinee":10387,"source_line":987,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":116620,"parent_record_offset":116596,"ancestor_record_offset":46340,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":56804,"parent_record_offset":56540,"ancestor_record_offset":46340,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":58088,"parent_record_offset":58052,"ancestor_record_offset":46340,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$89::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":91460,"parent_record_offset":91424,"ancestor_record_offset":46340,"inlinee":10397,"source_line":135,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$98::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":91492,"parent_record_offset":91360,"ancestor_record_offset":46340,"inlinee":10399,"source_line":371,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":1686,"record_offset":118388,"parent_record_offset":118352,"ancestor_record_offset":46340,"inlinee":7806,"source_line":197,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\counter.rs
//     evidence={"module":1686,"record_offset":118424,"parent_record_offset":118388,"ancestor_record_offset":46340,"inlinee":7808,"source_line":20,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpsc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":1686,"record_offset":118352,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":7841,"source_line":519,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1686,"record_offset":116676,"parent_record_offset":116648,"ancestor_record_offset":46340,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1686,"record_offset":55740,"parent_record_offset":55708,"ancestor_record_offset":46340,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1686,"record_offset":115708,"parent_record_offset":115676,"ancestor_record_offset":46340,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":1686,"record_offset":116700,"parent_record_offset":116676,"ancestor_record_offset":46340,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":1686,"record_offset":118664,"parent_record_offset":102452,"ancestor_record_offset":46340,"inlinee":12559,"source_line":289,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1686,"record_offset":55636,"parent_record_offset":55600,"ancestor_record_offset":46340,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1686,"record_offset":55600,"parent_record_offset":55564,"ancestor_record_offset":46340,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$22::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":1686,"record_offset":87144,"parent_record_offset":87120,"ancestor_record_offset":46340,"inlinee":10735,"source_line":437,"archive":"_pdb/B/modules/01686.jsonl.gz"}
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
// 首条位置: {"module":1686,"record_offset":33488,"rva":24249984,"va":5392959104}
// 编译实例 RVAs: 0x1720680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-10315773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8039862ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5392960296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Add$LT$core..time..Duration$GT$$GT$::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10211152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8b03064ff","size":5,"soft_fail":false,"target_va":5382748704,"va":5392959851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140052a30
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23914935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8491693fe","size":5,"soft_fail":false,"target_va":5369047600,"va":5392962530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23358392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e848949bfe","size":5,"soft_fail":false,"target_va":5369600800,"va":5392959187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22805085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8a305a4fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5392963208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140603bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17948182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8ea21eefe","size":5,"soft_fail":false,"target_va":5375015856,"va":5392964033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=24
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17879157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e88b2feffe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392960656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17444084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e80cd3f5fe","size":5,"soft_fail":false,"target_va":5375516032,"va":5392960111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140877550
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15376087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8296115ff","size":5,"soft_fail":false,"target_va":5377586512,"va":5392962594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140878f10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15369483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8f57a15ff","size":5,"soft_fail":false,"target_va":5377593104,"va":5392962582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140891060
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15270636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e814fd16ff","size":5,"soft_fail":false,"target_va":5377691744,"va":5392962375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408da430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14968823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e809981bff","size":5,"soft_fail":false,"target_va":5377991728,"va":5392960546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0d840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12661169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e84fce3eff","size":5,"soft_fail":false,"target_va":5380298816,"va":5392959980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5ce40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12338639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e831ba43ff","size":5,"soft_fail":false,"target_va":5380623936,"va":5392962570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b70a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12258253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e833f444ff","size":5,"soft_fail":false,"target_va":5380704880,"va":5392963128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b78be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12223215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8117d45ff","size":5,"soft_fail":false,"target_va":5380738016,"va":5392961226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10218864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8901264ff","size":5,"soft_fail":false,"target_va":5382740736,"va":5392959595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10131847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8796665ff","size":5,"soft_fail":false,"target_va":5382827728,"va":5392959570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e84a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4426258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8ee75bcff","size":5,"soft_fail":false,"target_va":5388534944,"va":5392961197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f3c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4377853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e80333bdff","size":5,"soft_fail":false,"target_va":5388581984,"va":5392959832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f86a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4359533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8937abdff","size":5,"soft_fail":false,"target_va":5388600992,"va":5392960520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14152d950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2045422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e812cae0ff","size":5,"soft_fail":false,"target_va":5390915920,"va":5392961337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141720490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e810fcffff","size":5,"soft_fail":false,"target_va":5392958608,"va":5392959611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18887129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8d9312001","size":5,"soft_fail":false,"target_va":5411847152,"va":5392960018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18885021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e89d292001","size":5,"soft_fail":false,"target_va":5411847168,"va":5392962142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18604327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8d91ee4fe","size":5,"soft_fail":false,"target_va":5374355376,"va":5392959698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18579414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8d67f1b01","size":5,"soft_fail":false,"target_va":5411542064,"va":5392962645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18579117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8ad7e1b01","size":5,"soft_fail":false,"target_va":5411541904,"va":5392962782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::save::save_game::AsyncSaveCmd> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e82acbffff","size":5,"soft_fail":false,"target_va":5392950064,"va":5392963585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12431430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8ba4f42ff","size":5,"soft_fail":false,"target_va":5380530176,"va":5392961601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::CurrentSaveFile::change_section
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e85bdbffff","size":5,"soft_fail":false,"target_va":5392952544,"va":5392961920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e881e5ffff","size":5,"soft_fail":false,"target_va":5392955376,"va":5392962154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::UnsavedFileList::contains_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e816f1ffff","size":5,"soft_fail":false,"target_va":5392957856,"va":5392961669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::execute_save_cmd
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8df120000","size":5,"soft_fail":false,"target_va":5392964256,"va":5392959420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::shadow_file::ShadowFile::sync_all
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14706189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8f3991fff","size":5,"soft_fail":false,"target_va":5378253360,"va":5392959544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18530306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e802c01a01","size":5,"soft_fail":false,"target_va":5411492368,"va":5392962057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18532709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e865c91a01","size":5,"soft_fail":false,"target_va":5411494800,"va":5392962086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11136222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8221356ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5392959817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11136906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8761056ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5392960917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10217568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8a01764ff","size":5,"soft_fail":false,"target_va":5382744080,"va":5392961643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-673894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e89ab7f5ff","size":5,"soft_fail":false,"target_va":5392287232,"va":5392961121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-693944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e84869f5ff","size":5,"soft_fail":false,"target_va":5392267168,"va":5392961107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-304148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8ec5bfbff","size":5,"soft_fail":false,"target_va":5392656992,"va":5392961135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18574303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8df6b1b01","size":5,"soft_fail":false,"target_va":5411537104,"va":5392962796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10270284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8b44963ff","size":5,"soft_fail":false,"target_va":5382691200,"va":5392961479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-10211631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392959104,"raw_hex":"e8d12e64ff","size":5,"soft_fail":false,"target_va":5382747616,"va":5392959242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1686,"record_offset":34256,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":34712,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":35152,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":34284,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":11236,"source_line":1407,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":35804,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1686,"record_offset":34684,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1686,"record_offset":44392,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":34460,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":41444,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":41472,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":41680,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":36304,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":36276,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::AutosaveErrorState::is_ok
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":35660,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":76801,"source_line":328,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::impl$10::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":36220,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":76803,"source_line":322,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::impl$11::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":34228,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":76797,"source_line":322,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":34488,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":28716,"source_line":247,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":1686,"record_offset":41380,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":8819,"source_line":221,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1686,"record_offset":34936,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1686,"record_offset":35096,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":35836,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":35720,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":33744,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":4784,"source_line":144,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":39520,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":15886,"source_line":964,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":40292,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":25284,"source_line":341,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":40584,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":40444,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":34656,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":13531,"source_line":2045,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":1686,"record_offset":39456,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":7838,"source_line":785,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":1686,"record_offset":36248,"parent_record_offset":33488,"ancestor_record_offset":33488,"inlinee":7835,"source_line":607,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":36720,"parent_record_offset":36696,"ancestor_record_offset":33488,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":36696,"parent_record_offset":36632,"ancestor_record_offset":33488,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":34788,"parent_record_offset":34756,"ancestor_record_offset":33488,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":36664,"parent_record_offset":36632,"ancestor_record_offset":33488,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":36632,"parent_record_offset":36604,"ancestor_record_offset":33488,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":34836,"parent_record_offset":34812,"ancestor_record_offset":33488,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":34812,"parent_record_offset":34788,"ancestor_record_offset":33488,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":36604,"parent_record_offset":36576,"ancestor_record_offset":33488,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1686,"record_offset":35492,"parent_record_offset":35468,"ancestor_record_offset":33488,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1686,"record_offset":43640,"parent_record_offset":43604,"ancestor_record_offset":33488,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1686,"record_offset":43604,"parent_record_offset":43580,"ancestor_record_offset":33488,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":34756,"parent_record_offset":34712,"ancestor_record_offset":33488,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":35688,"parent_record_offset":35660,"ancestor_record_offset":33488,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":36360,"parent_record_offset":36332,"ancestor_record_offset":33488,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1686,"record_offset":35200,"parent_record_offset":35176,"ancestor_record_offset":33488,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":34024,"parent_record_offset":34000,"ancestor_record_offset":33488,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":34076,"parent_record_offset":34052,"ancestor_record_offset":33488,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":34052,"parent_record_offset":34000,"ancestor_record_offset":33488,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":35120,"parent_record_offset":35096,"ancestor_record_offset":33488,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":35388,"parent_record_offset":35364,"ancestor_record_offset":33488,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":41524,"parent_record_offset":41500,"ancestor_record_offset":33488,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":35328,"parent_record_offset":35304,"ancestor_record_offset":33488,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":34388,"parent_record_offset":34364,"ancestor_record_offset":33488,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":35468,"parent_record_offset":35440,"ancestor_record_offset":33488,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1686,"record_offset":35304,"parent_record_offset":35280,"ancestor_record_offset":33488,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1686,"record_offset":35280,"parent_record_offset":35248,"ancestor_record_offset":33488,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":34100,"parent_record_offset":34076,"ancestor_record_offset":33488,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":37156,"parent_record_offset":37128,"ancestor_record_offset":33488,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":41408,"parent_record_offset":41380,"ancestor_record_offset":33488,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":35892,"parent_record_offset":35860,"ancestor_record_offset":33488,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":35920,"parent_record_offset":35860,"ancestor_record_offset":33488,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":33972,"parent_record_offset":33944,"ancestor_record_offset":33488,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=121
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":36332,"parent_record_offset":36304,"ancestor_record_offset":33488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":43672,"parent_record_offset":43640,"ancestor_record_offset":33488,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":41732,"parent_record_offset":41704,"ancestor_record_offset":33488,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":34124,"parent_record_offset":34100,"ancestor_record_offset":33488,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":35224,"parent_record_offset":35200,"ancestor_record_offset":33488,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":35248,"parent_record_offset":35224,"ancestor_record_offset":33488,"inlinee":5713,"source_line":57,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":35576,"parent_record_offset":35548,"ancestor_record_offset":33488,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":38408,"parent_record_offset":38344,"ancestor_record_offset":33488,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":35004,"parent_record_offset":34964,"ancestor_record_offset":33488,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":35744,"parent_record_offset":35720,"ancestor_record_offset":33488,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":38432,"parent_record_offset":38408,"ancestor_record_offset":33488,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":35028,"parent_record_offset":35004,"ancestor_record_offset":33488,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":35768,"parent_record_offset":35744,"ancestor_record_offset":33488,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":43700,"parent_record_offset":43640,"ancestor_record_offset":33488,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":43732,"parent_record_offset":43604,"ancestor_record_offset":33488,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":1686,"record_offset":34364,"parent_record_offset":34340,"ancestor_record_offset":33488,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":1686,"record_offset":35364,"parent_record_offset":35248,"ancestor_record_offset":33488,"inlinee":4992,"source_line":34,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":35176,"parent_record_offset":35152,"ancestor_record_offset":33488,"inlinee":76799,"source_line":322,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":35440,"parent_record_offset":35416,"ancestor_record_offset":33488,"inlinee":33006,"source_line":305,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::save_thread::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":41500,"parent_record_offset":41472,"ancestor_record_offset":33488,"inlinee":76804,"source_line":499,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":39568,"parent_record_offset":39544,"ancestor_record_offset":33488,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":39596,"parent_record_offset":39568,"ancestor_record_offset":33488,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":34184,"parent_record_offset":33828,"ancestor_record_offset":33488,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":33828,"parent_record_offset":33800,"ancestor_record_offset":33488,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":39624,"parent_record_offset":39596,"ancestor_record_offset":33488,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":43508,"parent_record_offset":43484,"ancestor_record_offset":33488,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1686,"record_offset":39544,"parent_record_offset":39520,"ancestor_record_offset":33488,"inlinee":15887,"source_line":1175,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::retain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1686,"record_offset":40316,"parent_record_offset":40292,"ancestor_record_offset":33488,"inlinee":25285,"source_line":368,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1686,"record_offset":33800,"parent_record_offset":33772,"ancestor_record_offset":33488,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1686,"record_offset":38316,"parent_record_offset":38288,"ancestor_record_offset":33488,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":35948,"parent_record_offset":35860,"ancestor_record_offset":33488,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":35860,"parent_record_offset":35836,"ancestor_record_offset":33488,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":35972,"parent_record_offset":35948,"ancestor_record_offset":33488,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1686,"record_offset":34964,"parent_record_offset":34936,"ancestor_record_offset":33488,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1686,"record_offset":38344,"parent_record_offset":38316,"ancestor_record_offset":33488,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1686,"record_offset":35060,"parent_record_offset":34964,"ancestor_record_offset":33488,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1686,"record_offset":38380,"parent_record_offset":38344,"ancestor_record_offset":33488,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":33772,"parent_record_offset":33744,"ancestor_record_offset":33488,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":34560,"parent_record_offset":34536,"ancestor_record_offset":33488,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":34608,"parent_record_offset":34584,"ancestor_record_offset":33488,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":34584,"parent_record_offset":34560,"ancestor_record_offset":33488,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1686,"record_offset":33884,"parent_record_offset":33856,"ancestor_record_offset":33488,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1686,"record_offset":34000,"parent_record_offset":33944,"ancestor_record_offset":33488,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1686,"record_offset":33856,"parent_record_offset":33828,"ancestor_record_offset":33488,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":34536,"parent_record_offset":34512,"ancestor_record_offset":33488,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":34512,"parent_record_offset":34488,"ancestor_record_offset":33488,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":1686,"record_offset":39480,"parent_record_offset":39456,"ancestor_record_offset":33488,"inlinee":7804,"source_line":915,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$20::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":1686,"record_offset":34340,"parent_record_offset":34316,"ancestor_record_offset":33488,"inlinee":5093,"source_line":14,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1686,"record_offset":33944,"parent_record_offset":33912,"ancestor_record_offset":33488,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1686,"record_offset":33912,"parent_record_offset":33884,"ancestor_record_offset":33488,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$23::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":1686,"record_offset":34316,"parent_record_offset":34284,"ancestor_record_offset":33488,"inlinee":5091,"source_line":149,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::CurrentSaveFile::replace_edits
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12C07
// 首条位置: {"module":1686,"inlinee":76807}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":75312,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":4915,"source_line":1524,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":75344,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":75408,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":75692,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":75036,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":75920,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::truncate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":78148,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":28711,"source_line":103,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::write_at
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":75072,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":28710,"source_line":93,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":78116,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":28698,"source_line":289,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFileWriter::write_all
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":77204,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":28721,"source_line":281,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":75728,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":75596,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":73932,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":72348,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":72844,"parent_record_offset":72148,"ancestor_record_offset":72148,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":76844,"parent_record_offset":76816,"ancestor_record_offset":72148,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":76816,"parent_record_offset":76716,"ancestor_record_offset":72148,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":77532,"parent_record_offset":77504,"ancestor_record_offset":72148,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":77668,"parent_record_offset":77636,"ancestor_record_offset":72148,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":77560,"parent_record_offset":77532,"ancestor_record_offset":72148,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":76748,"parent_record_offset":76716,"ancestor_record_offset":72148,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":76716,"parent_record_offset":76684,"ancestor_record_offset":72148,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":77732,"parent_record_offset":77700,"ancestor_record_offset":72148,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":77700,"parent_record_offset":77668,"ancestor_record_offset":72148,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":76684,"parent_record_offset":76656,"ancestor_record_offset":72148,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":77504,"parent_record_offset":77400,"ancestor_record_offset":72148,"inlinee":7415,"source_line":1246,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":77600,"parent_record_offset":77400,"ancestor_record_offset":72148,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":77636,"parent_record_offset":77400,"ancestor_record_offset":72148,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":77936,"parent_record_offset":77400,"ancestor_record_offset":72148,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":77808,"parent_record_offset":77400,"ancestor_record_offset":72148,"inlinee":8644,"source_line":2876,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":76344,"parent_record_offset":76304,"ancestor_record_offset":72148,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":76304,"parent_record_offset":76272,"ancestor_record_offset":72148,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":76628,"parent_record_offset":76604,"ancestor_record_offset":72148,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":76428,"parent_record_offset":76400,"ancestor_record_offset":72148,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":1686,"record_offset":78428,"parent_record_offset":78400,"ancestor_record_offset":72148,"inlinee":11197,"source_line":333,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":1686,"record_offset":78492,"parent_record_offset":78464,"ancestor_record_offset":72148,"inlinee":25488,"source_line":350,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":1686,"record_offset":78400,"parent_record_offset":78368,"ancestor_record_offset":72148,"inlinee":25242,"source_line":774,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::set_len_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec.rs
//     evidence={"module":1686,"record_offset":78464,"parent_record_offset":78368,"ancestor_record_offset":72148,"inlinee":25487,"source_line":585,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":1686,"record_offset":78368,"parent_record_offset":78148,"ancestor_record_offset":72148,"inlinee":28645,"source_line":359,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":74352,"parent_record_offset":74320,"ancestor_record_offset":72148,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":74408,"parent_record_offset":74380,"ancestor_record_offset":72148,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":74380,"parent_record_offset":74352,"ancestor_record_offset":72148,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":74288,"parent_record_offset":74252,"ancestor_record_offset":72148,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":74320,"parent_record_offset":74288,"ancestor_record_offset":72148,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":74512,"parent_record_offset":74320,"ancestor_record_offset":72148,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":74632,"parent_record_offset":74608,"ancestor_record_offset":72148,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":78028,"parent_record_offset":78000,"ancestor_record_offset":72148,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":75232,"parent_record_offset":75140,"ancestor_record_offset":72148,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1686,"record_offset":76400,"parent_record_offset":76372,"ancestor_record_offset":72148,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1686,"record_offset":76372,"parent_record_offset":76344,"ancestor_record_offset":72148,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":74436,"parent_record_offset":74408,"ancestor_record_offset":72148,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":78204,"parent_record_offset":78148,"ancestor_record_offset":72148,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":77472,"parent_record_offset":77400,"ancestor_record_offset":72148,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":76780,"parent_record_offset":76748,"ancestor_record_offset":72148,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":77760,"parent_record_offset":77732,"ancestor_record_offset":72148,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1686,"record_offset":72516,"parent_record_offset":72484,"ancestor_record_offset":72148,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":75140,"parent_record_offset":75112,"ancestor_record_offset":72148,"inlinee":14148,"source_line":593,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":78332,"parent_record_offset":78148,"ancestor_record_offset":72148,"inlinee":28676,"source_line":3197,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":75196,"parent_record_offset":75168,"ancestor_record_offset":72148,"inlinee":14150,"source_line":2350,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":75168,"parent_record_offset":75140,"ancestor_record_offset":72148,"inlinee":14149,"source_line":2431,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1686,"record_offset":74108,"parent_record_offset":74068,"ancestor_record_offset":72148,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":75788,"parent_record_offset":75756,"ancestor_record_offset":72148,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":75820,"parent_record_offset":75756,"ancestor_record_offset":72148,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":72788,"parent_record_offset":72760,"ancestor_record_offset":72148,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":74216,"parent_record_offset":74068,"ancestor_record_offset":72148,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":76512,"parent_record_offset":76484,"ancestor_record_offset":72148,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":74584,"parent_record_offset":74252,"ancestor_record_offset":72148,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":75372,"parent_record_offset":75344,"ancestor_record_offset":72148,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":78000,"parent_record_offset":77972,"ancestor_record_offset":72148,"inlinee":9440,"source_line":1455,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":76484,"parent_record_offset":76456,"ancestor_record_offset":72148,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":75496,"parent_record_offset":75408,"ancestor_record_offset":72148,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":73308,"parent_record_offset":73280,"ancestor_record_offset":72148,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":75528,"parent_record_offset":75408,"ancestor_record_offset":72148,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":73280,"parent_record_offset":73252,"ancestor_record_offset":72148,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":76456,"parent_record_offset":76428,"ancestor_record_offset":72148,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":73516,"parent_record_offset":73484,"ancestor_record_offset":72148,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":75556,"parent_record_offset":75528,"ancestor_record_offset":72148,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":74464,"parent_record_offset":74436,"ancestor_record_offset":72148,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":75272,"parent_record_offset":75072,"ancestor_record_offset":72148,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1686,"record_offset":77872,"parent_record_offset":77400,"ancestor_record_offset":72148,"inlinee":8645,"source_line":3567,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":73224,"parent_record_offset":73196,"ancestor_record_offset":72148,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":73252,"parent_record_offset":73224,"ancestor_record_offset":72148,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":73596,"parent_record_offset":73564,"ancestor_record_offset":72148,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":73196,"parent_record_offset":73072,"ancestor_record_offset":72148,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\specialize.rs
//     evidence={"module":1686,"record_offset":77900,"parent_record_offset":77872,"ancestor_record_offset":72148,"inlinee":8647,"source_line":18,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1686,"record_offset":72484,"parent_record_offset":72456,"ancestor_record_offset":72148,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1686,"record_offset":72876,"parent_record_offset":72844,"ancestor_record_offset":72148,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1686,"record_offset":73036,"parent_record_offset":73000,"ancestor_record_offset":72148,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1686,"record_offset":73000,"parent_record_offset":72960,"ancestor_record_offset":72148,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":72932,"parent_record_offset":72904,"ancestor_record_offset":72148,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":72960,"parent_record_offset":72932,"ancestor_record_offset":72148,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":72904,"parent_record_offset":72876,"ancestor_record_offset":72148,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":73564,"parent_record_offset":72960,"ancestor_record_offset":72148,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":72456,"parent_record_offset":72420,"ancestor_record_offset":72148,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":72420,"parent_record_offset":72392,"ancestor_record_offset":72148,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":72628,"parent_record_offset":72592,"ancestor_record_offset":72148,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":72732,"parent_record_offset":72628,"ancestor_record_offset":72148,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":72592,"parent_record_offset":72560,"ancestor_record_offset":72148,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":72392,"parent_record_offset":72348,"ancestor_record_offset":72148,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":72760,"parent_record_offset":72732,"ancestor_record_offset":72148,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":73072,"parent_record_offset":73036,"ancestor_record_offset":72148,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":73388,"parent_record_offset":73072,"ancestor_record_offset":72148,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":73356,"parent_record_offset":73072,"ancestor_record_offset":72148,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":73132,"parent_record_offset":73072,"ancestor_record_offset":72148,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":75624,"parent_record_offset":75596,"ancestor_record_offset":72148,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":75652,"parent_record_offset":75624,"ancestor_record_offset":72148,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":77244,"parent_record_offset":77204,"ancestor_record_offset":72148,"inlinee":28698,"source_line":289,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":75852,"parent_record_offset":75756,"ancestor_record_offset":72148,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":75756,"parent_record_offset":75728,"ancestor_record_offset":72148,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":75880,"parent_record_offset":75852,"ancestor_record_offset":72148,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":73976,"parent_record_offset":73932,"ancestor_record_offset":72148,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":78832,"parent_record_offset":78792,"ancestor_record_offset":72148,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":79440,"parent_record_offset":79140,"ancestor_record_offset":72148,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1686,"record_offset":74136,"parent_record_offset":74108,"ancestor_record_offset":72148,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":74252,"parent_record_offset":74068,"ancestor_record_offset":72148,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":74552,"parent_record_offset":74252,"ancestor_record_offset":72148,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Write::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":1686,"record_offset":77276,"parent_record_offset":77204,"ancestor_record_offset":72148,"inlinee":8631,"source_line":1754,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":75112,"parent_record_offset":75072,"ancestor_record_offset":72148,"inlinee":14156,"source_line":293,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$9::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":77308,"parent_record_offset":77276,"ancestor_record_offset":72148,"inlinee":28700,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":77400,"parent_record_offset":77340,"ancestor_record_offset":72148,"inlinee":8643,"source_line":443,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":77340,"parent_record_offset":77308,"ancestor_record_offset":72148,"inlinee":8642,"source_line":504,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":77972,"parent_record_offset":77340,"ancestor_record_offset":72148,"inlinee":8648,"source_line":486,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1686,"record_offset":74168,"parent_record_offset":74136,"ancestor_record_offset":72148,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1686,"record_offset":74068,"parent_record_offset":74020,"ancestor_record_offset":72148,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1686,"record_offset":74020,"parent_record_offset":73976,"ancestor_record_offset":72148,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01686.jsonl.gz"}
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
// 首条位置: {"module":1686,"record_offset":11984,"rva":24243424,"va":5392952544}
// 编译实例 RVAs: 0x171ece0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1790
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18274279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e81928e9fe","size":5,"soft_fail":false,"target_va":5374678928,"va":5392953202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17871974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e89a4beffe","size":5,"soft_fail":false,"target_va":5375081488,"va":5392953457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17872814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e85248effe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392954313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14091a450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14700243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e82db11fff","size":5,"soft_fail":false,"target_va":5378253904,"va":5392954142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141620f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1041067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e8551df0ff","size":5,"soft_fail":false,"target_va":5391912816,"va":5392953878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18715489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e861931d01","size":5,"soft_fail":false,"target_va":5411669968,"va":5392954474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18893627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e83b4b2001","size":5,"soft_fail":false,"target_va":5411847168,"va":5392953536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18893109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e835492001","size":5,"soft_fail":false,"target_va":5411847200,"va":5392954086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10113764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e81cad65ff","size":5,"soft_fail":false,"target_va":5382840208,"va":5392953967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18392357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e825a51801","size":5,"soft_fail":false,"target_va":5411347040,"va":5392954678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18586145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e8219a1b01","size":5,"soft_fail":false,"target_va":5411540963,"va":5392954813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18587047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e8a79d1b01","size":5,"soft_fail":false,"target_va":5411541584,"va":5392954532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18588308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e894a21b01","size":5,"soft_fail":false,"target_va":5411541808,"va":5392953495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18587809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e8a1a01b01","size":5,"soft_fail":false,"target_va":5411542422,"va":5392954608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18587495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e8679f1b01","size":5,"soft_fail":false,"target_va":5411542064,"va":5392954564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18588436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e814a31b01","size":5,"soft_fail":false,"target_va":5411543088,"va":5392954647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e8ecf9ffff","size":5,"soft_fail":false,"target_va":5392951088,"va":5392952639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18590646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e8b6ab1b01","size":5,"soft_fail":false,"target_va":5411543360,"va":5392952709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11567613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e8037e4fff","size":5,"soft_fail":false,"target_va":5381385536,"va":5392953144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11562424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e848924fff","size":5,"soft_fail":false,"target_va":5381390640,"va":5392953059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11567957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e8ab7c4fff","size":5,"soft_fail":false,"target_va":5381386480,"va":5392954432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11567903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e8e17c4fff","size":5,"soft_fail":false,"target_va":5381384704,"va":5392952602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18582579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392952544,"raw_hex":"e8338c1b01","size":5,"soft_fail":false,"target_va":5411537104,"va":5392954520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1686,"record_offset":17688,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":15788,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1686,"record_offset":17660,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":10612,"source_line":535,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":18632,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":17500,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":16392,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1686,"record_offset":17772,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":19492,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1686,"record_offset":15720,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":17584,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":15692,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":28712,"source_line":112,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFileWriter::write_all
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":17800,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":28721,"source_line":281,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":13636,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":12212,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":12660,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1686,"record_offset":16420,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":13598,"source_line":473,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$64::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1686,"record_offset":16796,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":14696,"source_line":2586,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1686,"record_offset":14620,"parent_record_offset":11984,"ancestor_record_offset":11984,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":16240,"parent_record_offset":16216,"ancestor_record_offset":11984,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":16264,"parent_record_offset":16240,"ancestor_record_offset":11984,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":19668,"parent_record_offset":19644,"ancestor_record_offset":11984,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":16216,"parent_record_offset":16040,"ancestor_record_offset":11984,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":19644,"parent_record_offset":19588,"ancestor_record_offset":11984,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1686,"record_offset":17736,"parent_record_offset":17712,"ancestor_record_offset":11984,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":18108,"parent_record_offset":18084,"ancestor_record_offset":11984,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":18228,"parent_record_offset":18200,"ancestor_record_offset":11984,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":15980,"parent_record_offset":15952,"ancestor_record_offset":11984,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":18132,"parent_record_offset":18108,"ancestor_record_offset":11984,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":19616,"parent_record_offset":19588,"ancestor_record_offset":11984,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":19588,"parent_record_offset":19564,"ancestor_record_offset":11984,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":18288,"parent_record_offset":18256,"ancestor_record_offset":11984,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":18256,"parent_record_offset":18228,"ancestor_record_offset":11984,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":16040,"parent_record_offset":16008,"ancestor_record_offset":11984,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":16008,"parent_record_offset":15980,"ancestor_record_offset":11984,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":19564,"parent_record_offset":19540,"ancestor_record_offset":11984,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":16076,"parent_record_offset":16040,"ancestor_record_offset":11984,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":15916,"parent_record_offset":15884,"ancestor_record_offset":11984,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":15884,"parent_record_offset":15852,"ancestor_record_offset":11984,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":15820,"parent_record_offset":15788,"ancestor_record_offset":11984,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":15852,"parent_record_offset":15820,"ancestor_record_offset":11984,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":17548,"parent_record_offset":17524,"ancestor_record_offset":11984,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":18084,"parent_record_offset":17984,"ancestor_record_offset":11984,"inlinee":7415,"source_line":1246,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":18168,"parent_record_offset":17984,"ancestor_record_offset":11984,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":18200,"parent_record_offset":17984,"ancestor_record_offset":11984,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":18468,"parent_record_offset":17984,"ancestor_record_offset":11984,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":18356,"parent_record_offset":17984,"ancestor_record_offset":11984,"inlinee":8644,"source_line":2876,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":15952,"parent_record_offset":15916,"ancestor_record_offset":11984,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":17524,"parent_record_offset":17500,"ancestor_record_offset":11984,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1686,"record_offset":16100,"parent_record_offset":16076,"ancestor_record_offset":11984,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1686,"record_offset":16124,"parent_record_offset":16100,"ancestor_record_offset":11984,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":13972,"parent_record_offset":13944,"ancestor_record_offset":11984,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":14024,"parent_record_offset":14000,"ancestor_record_offset":11984,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":14000,"parent_record_offset":13972,"ancestor_record_offset":11984,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":13920,"parent_record_offset":13892,"ancestor_record_offset":11984,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":13944,"parent_record_offset":13920,"ancestor_record_offset":11984,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":14116,"parent_record_offset":13944,"ancestor_record_offset":11984,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":14228,"parent_record_offset":14204,"ancestor_record_offset":11984,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":15444,"parent_record_offset":15420,"ancestor_record_offset":11984,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1686,"record_offset":16676,"parent_record_offset":16648,"ancestor_record_offset":11984,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1686,"record_offset":14952,"parent_record_offset":14928,"ancestor_record_offset":11984,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1686,"record_offset":15036,"parent_record_offset":15012,"ancestor_record_offset":11984,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":18552,"parent_record_offset":18524,"ancestor_record_offset":11984,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":14976,"parent_record_offset":14952,"ancestor_record_offset":11984,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":15512,"parent_record_offset":14816,"ancestor_record_offset":11984,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":16172,"parent_record_offset":16148,"ancestor_record_offset":11984,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":14048,"parent_record_offset":14024,"ancestor_record_offset":11984,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":16148,"parent_record_offset":16124,"ancestor_record_offset":11984,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":18056,"parent_record_offset":17984,"ancestor_record_offset":11984,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":18312,"parent_record_offset":18288,"ancestor_record_offset":11984,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1686,"record_offset":12360,"parent_record_offset":12328,"ancestor_record_offset":11984,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1686,"record_offset":15184,"parent_record_offset":15156,"ancestor_record_offset":11984,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1686,"record_offset":15156,"parent_record_offset":15128,"ancestor_record_offset":11984,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1686,"record_offset":13788,"parent_record_offset":13744,"ancestor_record_offset":11984,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":17712,"parent_record_offset":17688,"ancestor_record_offset":11984,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":12608,"parent_record_offset":12580,"ancestor_record_offset":11984,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":14576,"parent_record_offset":13744,"ancestor_record_offset":11984,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":16316,"parent_record_offset":15916,"ancestor_record_offset":11984,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":14180,"parent_record_offset":13892,"ancestor_record_offset":11984,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":18380,"parent_record_offset":18356,"ancestor_record_offset":11984,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":15304,"parent_record_offset":15280,"ancestor_record_offset":11984,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":13072,"parent_record_offset":13048,"ancestor_record_offset":11984,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":13048,"parent_record_offset":13024,"ancestor_record_offset":11984,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":13256,"parent_record_offset":13228,"ancestor_record_offset":11984,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":14072,"parent_record_offset":14048,"ancestor_record_offset":11984,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1686,"record_offset":18412,"parent_record_offset":17984,"ancestor_record_offset":11984,"inlinee":8645,"source_line":3567,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1686,"record_offset":15748,"parent_record_offset":15720,"ancestor_record_offset":11984,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":13000,"parent_record_offset":12976,"ancestor_record_offset":11984,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":13024,"parent_record_offset":13000,"ancestor_record_offset":11984,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":13328,"parent_record_offset":13300,"ancestor_record_offset":11984,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":12976,"parent_record_offset":12860,"ancestor_record_offset":11984,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\specialize.rs
//     evidence={"module":1686,"record_offset":18436,"parent_record_offset":18412,"ancestor_record_offset":11984,"inlinee":8647,"source_line":18,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1686,"record_offset":12328,"parent_record_offset":12300,"ancestor_record_offset":11984,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1686,"record_offset":12688,"parent_record_offset":12660,"ancestor_record_offset":11984,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1686,"record_offset":12828,"parent_record_offset":12796,"ancestor_record_offset":11984,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1686,"record_offset":12796,"parent_record_offset":12760,"ancestor_record_offset":11984,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":12736,"parent_record_offset":12712,"ancestor_record_offset":11984,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":12760,"parent_record_offset":12736,"ancestor_record_offset":11984,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":12712,"parent_record_offset":12688,"ancestor_record_offset":11984,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":13300,"parent_record_offset":12760,"ancestor_record_offset":11984,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":12300,"parent_record_offset":12268,"ancestor_record_offset":11984,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":12268,"parent_record_offset":12244,"ancestor_record_offset":11984,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":12460,"parent_record_offset":12428,"ancestor_record_offset":11984,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":12556,"parent_record_offset":12460,"ancestor_record_offset":11984,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":12428,"parent_record_offset":12404,"ancestor_record_offset":11984,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":12244,"parent_record_offset":12212,"ancestor_record_offset":11984,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":12580,"parent_record_offset":12556,"ancestor_record_offset":11984,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":12860,"parent_record_offset":12828,"ancestor_record_offset":11984,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":13144,"parent_record_offset":12860,"ancestor_record_offset":11984,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":13116,"parent_record_offset":12860,"ancestor_record_offset":11984,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":12920,"parent_record_offset":12860,"ancestor_record_offset":11984,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_game::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_game.rs
//     evidence={"module":1686,"record_offset":15468,"parent_record_offset":15444,"ancestor_record_offset":11984,"inlinee":10614,"source_line":479,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":17836,"parent_record_offset":17800,"ancestor_record_offset":11984,"inlinee":28698,"source_line":289,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1686,"record_offset":15096,"parent_record_offset":15068,"ancestor_record_offset":11984,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1686,"record_offset":15128,"parent_record_offset":15096,"ancestor_record_offset":11984,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1686,"record_offset":17148,"parent_record_offset":16960,"ancestor_record_offset":11984,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1686,"record_offset":15068,"parent_record_offset":14816,"ancestor_record_offset":11984,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1686,"record_offset":14928,"parent_record_offset":14816,"ancestor_record_offset":11984,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1686,"record_offset":16648,"parent_record_offset":16592,"ancestor_record_offset":11984,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1686,"record_offset":15540,"parent_record_offset":14816,"ancestor_record_offset":11984,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1686,"record_offset":16704,"parent_record_offset":16648,"ancestor_record_offset":11984,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1686,"record_offset":17240,"parent_record_offset":16912,"ancestor_record_offset":11984,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1686,"record_offset":15012,"parent_record_offset":14816,"ancestor_record_offset":11984,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1686,"record_offset":14900,"parent_record_offset":14816,"ancestor_record_offset":11984,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1686,"record_offset":15420,"parent_record_offset":15396,"ancestor_record_offset":11984,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":14684,"parent_record_offset":14648,"ancestor_record_offset":11984,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":14708,"parent_record_offset":14684,"ancestor_record_offset":11984,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":16476,"parent_record_offset":16448,"ancestor_record_offset":11984,"inlinee":13574,"source_line":799,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":15396,"parent_record_offset":15228,"ancestor_record_offset":11984,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":16852,"parent_record_offset":16824,"ancestor_record_offset":11984,"inlinee":11990,"source_line":3219,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":14740,"parent_record_offset":14708,"ancestor_record_offset":11984,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":15336,"parent_record_offset":15252,"ancestor_record_offset":11984,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":15280,"parent_record_offset":15252,"ancestor_record_offset":11984,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":17340,"parent_record_offset":16912,"ancestor_record_offset":11984,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":15632,"parent_record_offset":14816,"ancestor_record_offset":11984,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":16592,"parent_record_offset":16532,"ancestor_record_offset":11984,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":16912,"parent_record_offset":16880,"ancestor_record_offset":11984,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":15252,"parent_record_offset":15228,"ancestor_record_offset":11984,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":14792,"parent_record_offset":14768,"ancestor_record_offset":11984,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":14768,"parent_record_offset":14708,"ancestor_record_offset":11984,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":16504,"parent_record_offset":16476,"ancestor_record_offset":11984,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":16564,"parent_record_offset":16532,"ancestor_record_offset":11984,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":14816,"parent_record_offset":14792,"ancestor_record_offset":11984,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":16532,"parent_record_offset":16504,"ancestor_record_offset":11984,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":14872,"parent_record_offset":14816,"ancestor_record_offset":11984,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":16880,"parent_record_offset":16852,"ancestor_record_offset":11984,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":15228,"parent_record_offset":14816,"ancestor_record_offset":11984,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":13672,"parent_record_offset":13636,"ancestor_record_offset":11984,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":20212,"parent_record_offset":20180,"ancestor_record_offset":11984,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":20712,"parent_record_offset":20452,"ancestor_record_offset":11984,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1686,"record_offset":13816,"parent_record_offset":13788,"ancestor_record_offset":11984,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":13892,"parent_record_offset":13744,"ancestor_record_offset":11984,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":14152,"parent_record_offset":13892,"ancestor_record_offset":11984,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1686,"record_offset":14648,"parent_record_offset":14620,"ancestor_record_offset":11984,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1686,"record_offset":16448,"parent_record_offset":16420,"ancestor_record_offset":11984,"inlinee":13573,"source_line":570,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$50::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1686,"record_offset":16824,"parent_record_offset":16796,"ancestor_record_offset":11984,"inlinee":14698,"source_line":2428,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Write::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":1686,"record_offset":17864,"parent_record_offset":17800,"ancestor_record_offset":11984,"inlinee":8631,"source_line":1754,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$9::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":17896,"parent_record_offset":17864,"ancestor_record_offset":11984,"inlinee":28700,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":17984,"parent_record_offset":17924,"ancestor_record_offset":11984,"inlinee":8643,"source_line":443,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":17924,"parent_record_offset":17896,"ancestor_record_offset":11984,"inlinee":8642,"source_line":504,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":18500,"parent_record_offset":17924,"ancestor_record_offset":11984,"inlinee":8648,"source_line":486,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1686,"record_offset":13844,"parent_record_offset":13816,"ancestor_record_offset":11984,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1686,"record_offset":13744,"parent_record_offset":13708,"ancestor_record_offset":11984,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1686,"record_offset":13708,"parent_record_offset":13672,"ancestor_record_offset":11984,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01686.jsonl.gz"}
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
// 首条位置: {"module":1686,"record_offset":22688,"rva":24246256,"va":5392955376}
// 编译实例 RVAs: 0x171f7f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17874605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e85341effe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392956104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18713501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e89d8b1d01","size":5,"soft_fail":false,"target_va":5411669968,"va":5392956462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18584752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e8b0941b01","size":5,"soft_fail":false,"target_va":5411541584,"va":5392956827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e8fceeffff","size":5,"soft_fail":false,"target_va":5392951088,"va":5392955439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18587846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e8c6a01b01","size":5,"soft_fail":false,"target_va":5411543360,"va":5392955509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_thread::CurrentSaveFile::change_section
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e82bf2ffff","size":5,"soft_fail":false,"target_va":5392952544,"va":5392956080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::shadow_file::ShadowFile::sync_all
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14703220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e88ca51fff","size":5,"soft_fail":false,"target_va":5378253360,"va":5392956575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11570410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e816734fff","size":5,"soft_fail":false,"target_va":5381385536,"va":5392955941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11565224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e858874fff","size":5,"soft_fail":false,"target_va":5381390640,"va":5392955859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11570192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e8f0734fff","size":5,"soft_fail":false,"target_va":5381386480,"va":5392956667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11570711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e8e9714fff","size":5,"soft_fail":false,"target_va":5381384704,"va":5392955410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4131711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e881f4c0ff","size":5,"soft_fail":false,"target_va":5388825296,"va":5392957002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18580284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392955376,"raw_hex":"e83c831b01","size":5,"soft_fail":false,"target_va":5411537104,"va":5392956815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":25340,"parent_record_offset":22688,"ancestor_record_offset":22688,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":25456,"parent_record_offset":22688,"ancestor_record_offset":22688,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":1686,"record_offset":25276,"parent_record_offset":22688,"ancestor_record_offset":22688,"inlinee":8819,"source_line":221,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":24308,"parent_record_offset":22688,"ancestor_record_offset":22688,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":22876,"parent_record_offset":22688,"ancestor_record_offset":22688,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":23332,"parent_record_offset":22688,"ancestor_record_offset":22688,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":25656,"parent_record_offset":25632,"ancestor_record_offset":22688,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":25632,"parent_record_offset":25576,"ancestor_record_offset":22688,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":25604,"parent_record_offset":25576,"ancestor_record_offset":22688,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":25576,"parent_record_offset":25552,"ancestor_record_offset":22688,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":25552,"parent_record_offset":25528,"ancestor_record_offset":22688,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":24628,"parent_record_offset":24600,"ancestor_record_offset":22688,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":24680,"parent_record_offset":24656,"ancestor_record_offset":22688,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":24656,"parent_record_offset":24628,"ancestor_record_offset":22688,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":24576,"parent_record_offset":24548,"ancestor_record_offset":22688,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":24600,"parent_record_offset":24576,"ancestor_record_offset":22688,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":24772,"parent_record_offset":24600,"ancestor_record_offset":22688,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":24884,"parent_record_offset":24860,"ancestor_record_offset":22688,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":24704,"parent_record_offset":24680,"ancestor_record_offset":22688,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1686,"record_offset":23032,"parent_record_offset":23000,"ancestor_record_offset":22688,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1686,"record_offset":24444,"parent_record_offset":24404,"ancestor_record_offset":22688,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":25304,"parent_record_offset":25276,"ancestor_record_offset":22688,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":23280,"parent_record_offset":23252,"ancestor_record_offset":22688,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":25232,"parent_record_offset":24404,"ancestor_record_offset":22688,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":24836,"parent_record_offset":24548,"ancestor_record_offset":22688,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":23744,"parent_record_offset":23720,"ancestor_record_offset":22688,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":23720,"parent_record_offset":23696,"ancestor_record_offset":22688,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":23928,"parent_record_offset":23900,"ancestor_record_offset":22688,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":25392,"parent_record_offset":25364,"ancestor_record_offset":22688,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":24728,"parent_record_offset":24704,"ancestor_record_offset":22688,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":23672,"parent_record_offset":23648,"ancestor_record_offset":22688,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":23696,"parent_record_offset":23672,"ancestor_record_offset":22688,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":24000,"parent_record_offset":23972,"ancestor_record_offset":22688,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":23648,"parent_record_offset":23532,"ancestor_record_offset":22688,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1686,"record_offset":23000,"parent_record_offset":22972,"ancestor_record_offset":22688,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1686,"record_offset":23360,"parent_record_offset":23332,"ancestor_record_offset":22688,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1686,"record_offset":23500,"parent_record_offset":23468,"ancestor_record_offset":22688,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1686,"record_offset":23468,"parent_record_offset":23432,"ancestor_record_offset":22688,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":23408,"parent_record_offset":23384,"ancestor_record_offset":22688,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":23432,"parent_record_offset":23408,"ancestor_record_offset":22688,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":23384,"parent_record_offset":23360,"ancestor_record_offset":22688,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1686,"record_offset":23972,"parent_record_offset":23432,"ancestor_record_offset":22688,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":22972,"parent_record_offset":22940,"ancestor_record_offset":22688,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":22940,"parent_record_offset":22912,"ancestor_record_offset":22688,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":23132,"parent_record_offset":23100,"ancestor_record_offset":22688,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":23228,"parent_record_offset":23132,"ancestor_record_offset":22688,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":23100,"parent_record_offset":23076,"ancestor_record_offset":22688,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":22912,"parent_record_offset":22876,"ancestor_record_offset":22688,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":23252,"parent_record_offset":23228,"ancestor_record_offset":22688,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":23532,"parent_record_offset":23500,"ancestor_record_offset":22688,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":23816,"parent_record_offset":23532,"ancestor_record_offset":22688,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":23788,"parent_record_offset":23532,"ancestor_record_offset":22688,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1686,"record_offset":23592,"parent_record_offset":23532,"ancestor_record_offset":22688,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":24340,"parent_record_offset":24308,"ancestor_record_offset":22688,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":27764,"parent_record_offset":27728,"ancestor_record_offset":22688,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":28328,"parent_record_offset":28056,"ancestor_record_offset":22688,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1686,"record_offset":24472,"parent_record_offset":24444,"ancestor_record_offset":22688,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":24548,"parent_record_offset":24404,"ancestor_record_offset":22688,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1686,"record_offset":24808,"parent_record_offset":24548,"ancestor_record_offset":22688,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1686,"record_offset":24500,"parent_record_offset":24472,"ancestor_record_offset":22688,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1686,"record_offset":24404,"parent_record_offset":24372,"ancestor_record_offset":22688,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1686,"record_offset":24372,"parent_record_offset":24340,"ancestor_record_offset":22688,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::CurrentSaveFile::backup_save
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12C06
// 首条位置: {"module":1686,"inlinee":76806}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1686,"record_offset":57428,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":62080,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":62272,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":59552,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":59240,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1686,"record_offset":59272,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1686,"record_offset":61928,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1686,"record_offset":57512,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1686,"record_offset":62016,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":62444,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":56976,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":59128,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":1686,"record_offset":61548,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":32582,"source_line":54,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":59300,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":28712,"source_line":112,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":62776,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":59156,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":61000,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":58052,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":12455,"source_line":2523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":62968,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":8068,"source_line":2342,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":59328,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":56944,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":56804,"parent_record_offset":56540,"ancestor_record_offset":56540,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":57224,"parent_record_offset":57200,"ancestor_record_offset":56540,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":57200,"parent_record_offset":57144,"ancestor_record_offset":56540,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1686,"record_offset":64056,"parent_record_offset":64028,"ancestor_record_offset":56540,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1686,"record_offset":57476,"parent_record_offset":57452,"ancestor_record_offset":56540,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":62340,"parent_record_offset":62300,"ancestor_record_offset":56540,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":57172,"parent_record_offset":57144,"ancestor_record_offset":56540,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":57144,"parent_record_offset":57120,"ancestor_record_offset":56540,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":62396,"parent_record_offset":62368,"ancestor_record_offset":56540,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":62368,"parent_record_offset":62340,"ancestor_record_offset":56540,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":57120,"parent_record_offset":57096,"ancestor_record_offset":56540,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":62116,"parent_record_offset":62080,"ancestor_record_offset":56540,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":62300,"parent_record_offset":62272,"ancestor_record_offset":56540,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":62044,"parent_record_offset":62016,"ancestor_record_offset":56540,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1686,"record_offset":59632,"parent_record_offset":59588,"ancestor_record_offset":56540,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":59588,"parent_record_offset":59552,"ancestor_record_offset":56540,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":61672,"parent_record_offset":61548,"ancestor_record_offset":56540,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1686,"record_offset":64088,"parent_record_offset":64056,"ancestor_record_offset":56540,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1686,"record_offset":58256,"parent_record_offset":58052,"ancestor_record_offset":56540,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":62196,"parent_record_offset":62152,"ancestor_record_offset":56540,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":64116,"parent_record_offset":64088,"ancestor_record_offset":56540,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":57452,"parent_record_offset":57428,"ancestor_record_offset":56540,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":62836,"parent_record_offset":62804,"ancestor_record_offset":56540,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":62868,"parent_record_offset":62804,"ancestor_record_offset":56540,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=102
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":57000,"parent_record_offset":56976,"ancestor_record_offset":56540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":63836,"parent_record_offset":63804,"ancestor_record_offset":56540,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":63892,"parent_record_offset":63864,"ancestor_record_offset":56540,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":63864,"parent_record_offset":63836,"ancestor_record_offset":56540,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":61628,"parent_record_offset":61592,"ancestor_record_offset":56540,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1686,"record_offset":62152,"parent_record_offset":62116,"ancestor_record_offset":56540,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":59180,"parent_record_offset":59156,"ancestor_record_offset":56540,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":59204,"parent_record_offset":59180,"ancestor_record_offset":56540,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1686,"record_offset":61024,"parent_record_offset":61000,"ancestor_record_offset":56540,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":62900,"parent_record_offset":62804,"ancestor_record_offset":56540,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":62804,"parent_record_offset":62776,"ancestor_record_offset":56540,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":62928,"parent_record_offset":62900,"ancestor_record_offset":56540,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":56852,"parent_record_offset":56828,"ancestor_record_offset":56540,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":56900,"parent_record_offset":56876,"ancestor_record_offset":56540,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":56876,"parent_record_offset":56852,"ancestor_record_offset":56540,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":61592,"parent_record_offset":61548,"ancestor_record_offset":56540,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":63804,"parent_record_offset":63772,"ancestor_record_offset":56540,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":63772,"parent_record_offset":63740,"ancestor_record_offset":56540,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":56828,"parent_record_offset":56804,"ancestor_record_offset":56540,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":58112,"parent_record_offset":58088,"ancestor_record_offset":56540,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":58088,"parent_record_offset":58052,"ancestor_record_offset":56540,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$0::backup_save::closure$1
// 记录形式: inlinee_declaration; instances=1
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8095
// 首条位置: {"module":576,"inlinee":32917}
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
// 记录形式: inline_instance, inlinee_declaration; instances=35
// 源码行: 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x803B
// 首条位置: {"module":574,"inlinee":32827}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ordering::reverse
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1592,"record_offset":128444,"parent_record_offset":128424,"ancestor_record_offset":128424,"inlinee":10270,"source_line":547,"archive":"_pdb/B/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] std::time::impl$32::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":1654,"record_offset":20028,"parent_record_offset":20008,"ancestor_record_offset":20008,"inlinee":75102,"source_line":242,"archive":"_pdb/B/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::SystemTime::intervals
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":1654,"record_offset":20076,"parent_record_offset":20052,"ancestor_record_offset":20008,"inlinee":28246,"source_line":84,"archive":"_pdb/B/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$5::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":1654,"record_offset":20052,"parent_record_offset":20028,"ancestor_record_offset":20008,"inlinee":16241,"source_line":124,"archive":"_pdb/B/modules/01654.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::CurrentSaveFile::perform_manual_backup
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12C0A
// 首条位置: {"module":1686,"inlinee":76810}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":87880,"parent_record_offset":87232,"ancestor_record_offset":87232,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1686,"record_offset":87912,"parent_record_offset":87232,"ancestor_record_offset":87232,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":87484,"parent_record_offset":87232,"ancestor_record_offset":87232,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":87940,"parent_record_offset":87232,"ancestor_record_offset":87232,"inlinee":28712,"source_line":112,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":87796,"parent_record_offset":87232,"ancestor_record_offset":87232,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":87968,"parent_record_offset":87232,"ancestor_record_offset":87232,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":87452,"parent_record_offset":87232,"ancestor_record_offset":87232,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":87312,"parent_record_offset":87232,"ancestor_record_offset":87232,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":87732,"parent_record_offset":87708,"ancestor_record_offset":87232,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":87708,"parent_record_offset":87652,"ancestor_record_offset":87232,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":87680,"parent_record_offset":87652,"ancestor_record_offset":87232,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":87652,"parent_record_offset":87628,"ancestor_record_offset":87232,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":87628,"parent_record_offset":87604,"ancestor_record_offset":87232,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1686,"record_offset":87992,"parent_record_offset":87968,"ancestor_record_offset":87232,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":87508,"parent_record_offset":87484,"ancestor_record_offset":87232,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":87820,"parent_record_offset":87796,"ancestor_record_offset":87232,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":87844,"parent_record_offset":87820,"ancestor_record_offset":87232,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":87360,"parent_record_offset":87336,"ancestor_record_offset":87232,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":87408,"parent_record_offset":87384,"ancestor_record_offset":87232,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":87384,"parent_record_offset":87360,"ancestor_record_offset":87232,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":87336,"parent_record_offset":87312,"ancestor_record_offset":87232,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":88040,"parent_record_offset":88016,"ancestor_record_offset":87232,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":88016,"parent_record_offset":87992,"ancestor_record_offset":87232,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$0::perform_manual_backup::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 273
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x121EE
// 首条位置: {"module":1629,"inlinee":74222}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1629,"record_offset":108180,"parent_record_offset":108016,"ancestor_record_offset":108016,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01629.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1629,"record_offset":108040,"parent_record_offset":108016,"ancestor_record_offset":108016,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01629.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1629,"record_offset":108228,"parent_record_offset":108204,"ancestor_record_offset":108016,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01629.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1629,"record_offset":108204,"parent_record_offset":108180,"ancestor_record_offset":108016,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01629.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1629,"record_offset":108088,"parent_record_offset":108064,"ancestor_record_offset":108016,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01629.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1629,"record_offset":108136,"parent_record_offset":108112,"ancestor_record_offset":108016,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01629.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1629,"record_offset":108112,"parent_record_offset":108088,"ancestor_record_offset":108016,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01629.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1629,"record_offset":108064,"parent_record_offset":108040,"ancestor_record_offset":108016,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01629.jsonl.gz"}
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
// 首条位置: {"module":1686,"record_offset":30356,"rva":24248192,"va":5392957312}
// 编译实例 RVAs: 0x171ff80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22799638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957312,"raw_hex":"e8ea1aa4fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5392957761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088ed10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15275022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957312,"raw_hex":"e8f2eb16ff","size":5,"soft_fail":false,"target_va":5377682704,"va":5392957721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18602296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957312,"raw_hex":"e8c826e4fe","size":5,"soft_fail":false,"target_va":5374355184,"va":5392957475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18534736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957312,"raw_hex":"e850d11a01","size":5,"soft_fail":false,"target_va":5411492368,"va":5392957627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18537149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957312,"raw_hex":"e8bdda1a01","size":5,"soft_fail":false,"target_va":5411494800,"va":5392957646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11133779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957312,"raw_hex":"e8ad1c56ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5392957374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11133569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957312,"raw_hex":"e87f1d56ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5392957580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":30668,"parent_record_offset":30356,"ancestor_record_offset":30356,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":30868,"parent_record_offset":30356,"ancestor_record_offset":30356,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1686,"record_offset":30512,"parent_record_offset":30356,"ancestor_record_offset":30356,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":30748,"parent_record_offset":30716,"ancestor_record_offset":30356,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":30796,"parent_record_offset":30772,"ancestor_record_offset":30356,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":30772,"parent_record_offset":30748,"ancestor_record_offset":30356,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":30716,"parent_record_offset":30668,"ancestor_record_offset":30356,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":30836,"parent_record_offset":30668,"ancestor_record_offset":30356,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":30988,"parent_record_offset":30924,"ancestor_record_offset":30356,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":30576,"parent_record_offset":30540,"ancestor_record_offset":30356,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":31012,"parent_record_offset":30988,"ancestor_record_offset":30356,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":30600,"parent_record_offset":30576,"ancestor_record_offset":30356,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1686,"record_offset":30896,"parent_record_offset":30868,"ancestor_record_offset":30356,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1686,"record_offset":30540,"parent_record_offset":30512,"ancestor_record_offset":30356,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1686,"record_offset":30924,"parent_record_offset":30896,"ancestor_record_offset":30356,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1686,"record_offset":30632,"parent_record_offset":30540,"ancestor_record_offset":30356,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1686,"record_offset":30960,"parent_record_offset":30924,"ancestor_record_offset":30356,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01686.jsonl.gz"}
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
// 首条位置: {"module":1686,"record_offset":31060,"rva":24248736,"va":5392957856}
// 编译实例 RVAs: 0x17201a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22800456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957856,"raw_hex":"e8b817a4fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5392958579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10130260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957856,"raw_hex":"e8ac6c65ff","size":5,"soft_fail":false,"target_va":5382827728,"va":5392957983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f6360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-172090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957856,"raw_hex":"e8c65ffdff","size":5,"soft_fail":false,"target_va":5392786272,"va":5392958357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18888736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957856,"raw_hex":"e820382001","size":5,"soft_fail":false,"target_va":5411847152,"va":5392958411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18533849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957856,"raw_hex":"e8d9cd1a01","size":5,"soft_fail":false,"target_va":5411492368,"va":5392958514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18536266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957856,"raw_hex":"e84ad71a01","size":5,"soft_fail":false,"target_va":5411494800,"va":5392958529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11134333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957856,"raw_hex":"e8831a56ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5392957928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11134453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957856,"raw_hex":"e80b1a56ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5392958464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::components
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10212918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392957856,"raw_hex":"e8ca2964ff","size":5,"soft_fail":false,"target_va":5382745088,"va":5392958001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":32484,"parent_record_offset":31060,"ancestor_record_offset":31060,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":31380,"parent_record_offset":31060,"ancestor_record_offset":31060,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1686,"record_offset":31224,"parent_record_offset":31060,"ancestor_record_offset":31060,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":31684,"parent_record_offset":31488,"ancestor_record_offset":31060,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1686,"record_offset":31968,"parent_record_offset":31808,"ancestor_record_offset":31060,"inlinee":10401,"source_line":20,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1686,"record_offset":32212,"parent_record_offset":32176,"ancestor_record_offset":31060,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":32176,"parent_record_offset":32144,"ancestor_record_offset":31060,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":31872,"parent_record_offset":31808,"ancestor_record_offset":31060,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":31452,"parent_record_offset":31416,"ancestor_record_offset":31060,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":32404,"parent_record_offset":32380,"ancestor_record_offset":31060,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":32428,"parent_record_offset":32404,"ancestor_record_offset":31060,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":31416,"parent_record_offset":31380,"ancestor_record_offset":31060,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":32604,"parent_record_offset":32540,"ancestor_record_offset":31060,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":31288,"parent_record_offset":31252,"ancestor_record_offset":31060,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":32628,"parent_record_offset":32604,"ancestor_record_offset":31060,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":31312,"parent_record_offset":31288,"ancestor_record_offset":31060,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::impl$1::contains_file::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":31488,"parent_record_offset":31380,"ancestor_record_offset":31060,"inlinee":76795,"source_line":301,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::ShadowFile::path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":31516,"parent_record_offset":31488,"ancestor_record_offset":31060,"inlinee":28716,"source_line":247,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1686,"record_offset":32512,"parent_record_offset":32484,"ancestor_record_offset":31060,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1686,"record_offset":31252,"parent_record_offset":31224,"ancestor_record_offset":31060,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1686,"record_offset":32540,"parent_record_offset":32512,"ancestor_record_offset":31060,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1686,"record_offset":31344,"parent_record_offset":31252,"ancestor_record_offset":31060,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1686,"record_offset":32576,"parent_record_offset":32540,"ancestor_record_offset":31060,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":31588,"parent_record_offset":31564,"ancestor_record_offset":31060,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":31636,"parent_record_offset":31612,"ancestor_record_offset":31060,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":31612,"parent_record_offset":31588,"ancestor_record_offset":31060,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":32144,"parent_record_offset":31748,"ancestor_record_offset":31060,"inlinee":10391,"source_line":646,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31564,"parent_record_offset":31540,"ancestor_record_offset":31060,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Prefix::is_verbatim
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":32240,"parent_record_offset":32212,"ancestor_record_offset":31060,"inlinee":10393,"source_line":222,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$100::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":32116,"parent_record_offset":31748,"ancestor_record_offset":31060,"inlinee":10389,"source_line":371,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$116::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31808,"parent_record_offset":31748,"ancestor_record_offset":31060,"inlinee":10395,"source_line":589,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31748,"parent_record_offset":31716,"ancestor_record_offset":31060,"inlinee":10387,"source_line":987,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31540,"parent_record_offset":31516,"ancestor_record_offset":31060,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$72::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31716,"parent_record_offset":31684,"ancestor_record_offset":31060,"inlinee":32998,"source_line":3231,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$89::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31908,"parent_record_offset":31872,"ancestor_record_offset":31060,"inlinee":10397,"source_line":135,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$98::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31940,"parent_record_offset":31808,"ancestor_record_offset":31060,"inlinee":10399,"source_line":371,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$1::contains_file::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12BFB
// 首条位置: {"module":1686,"inlinee":76795}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":31684,"parent_record_offset":31488,"ancestor_record_offset":31488,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1686,"record_offset":31516,"parent_record_offset":31488,"ancestor_record_offset":31488,"inlinee":28716,"source_line":247,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":32380,"parent_record_offset":31748,"ancestor_record_offset":31488,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1686,"record_offset":31968,"parent_record_offset":31808,"ancestor_record_offset":31488,"inlinee":10401,"source_line":20,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1686,"record_offset":32212,"parent_record_offset":32176,"ancestor_record_offset":31488,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":32176,"parent_record_offset":32144,"ancestor_record_offset":31488,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":31872,"parent_record_offset":31808,"ancestor_record_offset":31488,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":32404,"parent_record_offset":32380,"ancestor_record_offset":31488,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":32428,"parent_record_offset":32404,"ancestor_record_offset":31488,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":31588,"parent_record_offset":31564,"ancestor_record_offset":31488,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":31636,"parent_record_offset":31612,"ancestor_record_offset":31488,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":31612,"parent_record_offset":31588,"ancestor_record_offset":31488,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":32144,"parent_record_offset":31748,"ancestor_record_offset":31488,"inlinee":10391,"source_line":646,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31564,"parent_record_offset":31540,"ancestor_record_offset":31488,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Prefix::is_verbatim
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":32240,"parent_record_offset":32212,"ancestor_record_offset":31488,"inlinee":10393,"source_line":222,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$100::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":32116,"parent_record_offset":31748,"ancestor_record_offset":31488,"inlinee":10389,"source_line":371,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$116::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31808,"parent_record_offset":31748,"ancestor_record_offset":31488,"inlinee":10395,"source_line":589,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31748,"parent_record_offset":31716,"ancestor_record_offset":31488,"inlinee":10387,"source_line":987,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31540,"parent_record_offset":31516,"ancestor_record_offset":31488,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$72::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31716,"parent_record_offset":31684,"ancestor_record_offset":31488,"inlinee":32998,"source_line":3231,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$89::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31908,"parent_record_offset":31872,"ancestor_record_offset":31488,"inlinee":10397,"source_line":135,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$98::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":31940,"parent_record_offset":31808,"ancestor_record_offset":31488,"inlinee":10399,"source_line":371,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$5::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 305
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A92
// 首条位置: {"module":1585,"inlinee":72338}
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
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 305
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x80EE
// 首条位置: {"module":577,"inlinee":33006}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":35468,"parent_record_offset":35440,"ancestor_record_offset":35440,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1686,"record_offset":35492,"parent_record_offset":35468,"ancestor_record_offset":35440,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1686,"record_offset":35520,"parent_record_offset":35492,"ancestor_record_offset":35440,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":35548,"parent_record_offset":35520,"ancestor_record_offset":35440,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":35576,"parent_record_offset":35548,"ancestor_record_offset":35440,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01686.jsonl.gz"}
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
// 首条位置: {"module":1686,"record_offset":32676,"rva":24249488,"va":5392958608}
// 编译实例 RVAs: 0x1720490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10118686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392958608,"raw_hex":"e8e29965ff","size":5,"soft_fail":false,"target_va":5382840208,"va":5392958889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392958608,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5392958650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1686,"record_offset":33084,"parent_record_offset":32676,"ancestor_record_offset":32676,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] anyhow::Error::downcast_ref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1686,"record_offset":32828,"parent_record_offset":32676,"ancestor_record_offset":32676,"inlinee":31449,"source_line":506,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::Error::kind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error.rs
//     evidence={"module":1686,"record_offset":32884,"parent_record_offset":32676,"ancestor_record_offset":32676,"inlinee":12460,"source_line":992,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::impl$20::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error.rs
//     evidence={"module":1686,"record_offset":33052,"parent_record_offset":32676,"ancestor_record_offset":32676,"inlinee":8610,"source_line":219,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1686,"record_offset":33140,"parent_record_offset":33112,"ancestor_record_offset":32676,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::error::vtable
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1686,"record_offset":32852,"parent_record_offset":32828,"ancestor_record_offset":32676,"inlinee":9078,"source_line":880,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":33112,"parent_record_offset":33084,"ancestor_record_offset":32676,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::data
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":32916,"parent_record_offset":32884,"ancestor_record_offset":32676,"inlinee":8608,"source_line":217,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":32948,"parent_record_offset":32916,"ancestor_record_offset":32676,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::kind_from_prim
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":32984,"parent_record_offset":32948,"ancestor_record_offset":32676,"inlinee":8613,"source_line":299,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::decode_error_kind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\mod.rs
//     evidence={"module":1686,"record_offset":33020,"parent_record_offset":32884,"ancestor_record_offset":32676,"inlinee":12428,"source_line":72,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::impl$10::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12C03
// 首条位置: {"module":1686,"inlinee":76803}
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
// IPI IDs: 0x12BFD
// 首条位置: {"module":1686,"inlinee":76797}
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
// IPI IDs: 0x12BFF
// 首条位置: {"module":1686,"inlinee":76799}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1686,"record_offset":35200,"parent_record_offset":35176,"ancestor_record_offset":35176,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1686,"record_offset":35492,"parent_record_offset":35468,"ancestor_record_offset":35176,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1686,"record_offset":35520,"parent_record_offset":35492,"ancestor_record_offset":35176,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":35388,"parent_record_offset":35364,"ancestor_record_offset":35176,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":35328,"parent_record_offset":35304,"ancestor_record_offset":35176,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":35468,"parent_record_offset":35440,"ancestor_record_offset":35176,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1686,"record_offset":35304,"parent_record_offset":35280,"ancestor_record_offset":35176,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1686,"record_offset":35280,"parent_record_offset":35248,"ancestor_record_offset":35176,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":35224,"parent_record_offset":35200,"ancestor_record_offset":35176,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":35248,"parent_record_offset":35224,"ancestor_record_offset":35176,"inlinee":5713,"source_line":57,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":35576,"parent_record_offset":35548,"ancestor_record_offset":35176,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":1686,"record_offset":35364,"parent_record_offset":35248,"ancestor_record_offset":35176,"inlinee":4992,"source_line":34,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":35440,"parent_record_offset":35416,"ancestor_record_offset":35176,"inlinee":33006,"source_line":305,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::AutosaveErrorState::is_ok
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 328, 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12C01
// 首条位置: {"module":1686,"inlinee":76801}
// 编译实例 RVAs: 0xb92210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":33460,"parent_record_offset":33308,"ancestor_record_offset":33308,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01686.jsonl.gz"}
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
// 首条位置: {"module":1585,"record_offset":491076,"rva":19986688,"va":5388695808}
// 编译实例 RVAs: 0x130f900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6051486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388695808,"raw_hex":"e862a9a3ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5388696009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13614750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388695808,"raw_hex":"e8624130ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388696249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13179860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388695808,"raw_hex":"e82ce436ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5388695887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b70a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7991423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388695808,"raw_hex":"e8810f86ff","size":5,"soft_fail":false,"target_va":5380704880,"va":5388696298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5955321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388695808,"raw_hex":"e80721a5ff","size":5,"soft_fail":false,"target_va":5382740736,"va":5388696052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5868314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388695808,"raw_hex":"e8e674a6ff","size":5,"soft_fail":false,"target_va":5382827728,"va":5388696037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141720490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4262539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388695808,"raw_hex":"e88b0a4100","size":5,"soft_fail":false,"target_va":5392958608,"va":5388696064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14340749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388695808,"raw_hex":"e8732d25ff","size":5,"soft_fail":false,"target_va":5374355376,"va":5388696120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::shadow_file::ShadowFile::sync_all
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10442491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388695808,"raw_hex":"e805a960ff","size":5,"soft_fail":false,"target_va":5378253360,"va":5388695846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":491760,"parent_record_offset":491076,"ancestor_record_offset":491076,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1585,"record_offset":491336,"parent_record_offset":491076,"ancestor_record_offset":491076,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1585,"record_offset":491732,"parent_record_offset":491076,"ancestor_record_offset":491076,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1585,"record_offset":491960,"parent_record_offset":491076,"ancestor_record_offset":491076,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":1585,"record_offset":491536,"parent_record_offset":491076,"ancestor_record_offset":491076,"inlinee":28716,"source_line":247,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1585,"record_offset":491368,"parent_record_offset":491076,"ancestor_record_offset":491076,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1585,"record_offset":491252,"parent_record_offset":491076,"ancestor_record_offset":491076,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1585,"record_offset":491704,"parent_record_offset":491076,"ancestor_record_offset":491076,"inlinee":13531,"source_line":2045,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1585,"record_offset":492208,"parent_record_offset":492184,"ancestor_record_offset":491076,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1585,"record_offset":492184,"parent_record_offset":492128,"ancestor_record_offset":491076,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":491840,"parent_record_offset":491808,"ancestor_record_offset":491076,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":492156,"parent_record_offset":492128,"ancestor_record_offset":491076,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":492128,"parent_record_offset":492104,"ancestor_record_offset":491076,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":491888,"parent_record_offset":491864,"ancestor_record_offset":491076,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":491864,"parent_record_offset":491840,"ancestor_record_offset":491076,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":492104,"parent_record_offset":492080,"ancestor_record_offset":491076,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":491808,"parent_record_offset":491760,"ancestor_record_offset":491076,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1585,"record_offset":491420,"parent_record_offset":491392,"ancestor_record_offset":491076,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1585,"record_offset":491448,"parent_record_offset":491392,"ancestor_record_offset":491076,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1585,"record_offset":491984,"parent_record_offset":491960,"ancestor_record_offset":491076,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1585,"record_offset":491928,"parent_record_offset":491760,"ancestor_record_offset":491076,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1585,"record_offset":491276,"parent_record_offset":491252,"ancestor_record_offset":491076,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1585,"record_offset":491300,"parent_record_offset":491276,"ancestor_record_offset":491076,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1585,"record_offset":491476,"parent_record_offset":491392,"ancestor_record_offset":491076,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1585,"record_offset":491392,"parent_record_offset":491368,"ancestor_record_offset":491076,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1585,"record_offset":491500,"parent_record_offset":491476,"ancestor_record_offset":491076,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1585,"record_offset":491608,"parent_record_offset":491584,"ancestor_record_offset":491076,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1585,"record_offset":491656,"parent_record_offset":491632,"ancestor_record_offset":491076,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1585,"record_offset":491632,"parent_record_offset":491608,"ancestor_record_offset":491076,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1585,"record_offset":491584,"parent_record_offset":491560,"ancestor_record_offset":491076,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1585,"record_offset":491560,"parent_record_offset":491536,"ancestor_record_offset":491076,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::save_thread::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 427
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120F4
// 首条位置: {"module":1613,"inlinee":73972}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1613,"record_offset":231108,"parent_record_offset":229572,"ancestor_record_offset":229572,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1613,"record_offset":230632,"parent_record_offset":229572,"ancestor_record_offset":229572,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1613,"record_offset":231456,"parent_record_offset":229572,"ancestor_record_offset":229572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1613,"record_offset":231280,"parent_record_offset":229572,"ancestor_record_offset":229572,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1613,"record_offset":231024,"parent_record_offset":229572,"ancestor_record_offset":229572,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1613,"record_offset":229636,"parent_record_offset":229572,"ancestor_record_offset":229572,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1613,"record_offset":230800,"parent_record_offset":229572,"ancestor_record_offset":229572,"inlinee":8068,"source_line":2342,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1613,"record_offset":230660,"parent_record_offset":229572,"ancestor_record_offset":229572,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1613,"record_offset":231884,"parent_record_offset":231860,"ancestor_record_offset":229572,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1613,"record_offset":231860,"parent_record_offset":231772,"ancestor_record_offset":229572,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1613,"record_offset":231772,"parent_record_offset":231744,"ancestor_record_offset":229572,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1613,"record_offset":231804,"parent_record_offset":231772,"ancestor_record_offset":229572,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1613,"record_offset":230328,"parent_record_offset":230304,"ancestor_record_offset":229572,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1613,"record_offset":230824,"parent_record_offset":230800,"ancestor_record_offset":229572,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1613,"record_offset":229892,"parent_record_offset":229868,"ancestor_record_offset":229572,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1613,"record_offset":229976,"parent_record_offset":229952,"ancestor_record_offset":229572,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1613,"record_offset":229916,"parent_record_offset":229892,"ancestor_record_offset":229572,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1613,"record_offset":230368,"parent_record_offset":229812,"ancestor_record_offset":229572,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1613,"record_offset":231828,"parent_record_offset":231804,"ancestor_record_offset":229572,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1613,"record_offset":230128,"parent_record_offset":230100,"ancestor_record_offset":229572,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1613,"record_offset":230100,"parent_record_offset":230072,"ancestor_record_offset":229572,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1613,"record_offset":231336,"parent_record_offset":231304,"ancestor_record_offset":229572,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1613,"record_offset":231364,"parent_record_offset":231304,"ancestor_record_offset":229572,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1613,"record_offset":231484,"parent_record_offset":231456,"ancestor_record_offset":229572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1613,"record_offset":230244,"parent_record_offset":230220,"ancestor_record_offset":229572,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1613,"record_offset":231572,"parent_record_offset":231544,"ancestor_record_offset":229572,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1613,"record_offset":231620,"parent_record_offset":231596,"ancestor_record_offset":229572,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1613,"record_offset":231596,"parent_record_offset":231572,"ancestor_record_offset":229572,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1613,"record_offset":231048,"parent_record_offset":231024,"ancestor_record_offset":229572,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1613,"record_offset":231072,"parent_record_offset":231048,"ancestor_record_offset":229572,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1613,"record_offset":230036,"parent_record_offset":230008,"ancestor_record_offset":229572,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1613,"record_offset":230072,"parent_record_offset":230036,"ancestor_record_offset":229572,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1613,"record_offset":230008,"parent_record_offset":229812,"ancestor_record_offset":229572,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1613,"record_offset":229868,"parent_record_offset":229812,"ancestor_record_offset":229572,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1613,"record_offset":230396,"parent_record_offset":229812,"ancestor_record_offset":229572,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1613,"record_offset":229952,"parent_record_offset":229812,"ancestor_record_offset":229572,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1613,"record_offset":230548,"parent_record_offset":229812,"ancestor_record_offset":229572,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1613,"record_offset":230304,"parent_record_offset":230280,"ancestor_record_offset":229572,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1613,"record_offset":229692,"parent_record_offset":229664,"ancestor_record_offset":229572,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1613,"record_offset":229720,"parent_record_offset":229692,"ancestor_record_offset":229572,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1613,"record_offset":230280,"parent_record_offset":230172,"ancestor_record_offset":229572,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1613,"record_offset":230588,"parent_record_offset":229720,"ancestor_record_offset":229572,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":230220,"parent_record_offset":230196,"ancestor_record_offset":229572,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":230196,"parent_record_offset":230172,"ancestor_record_offset":229572,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":230488,"parent_record_offset":229812,"ancestor_record_offset":229572,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":229784,"parent_record_offset":229756,"ancestor_record_offset":229572,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":229756,"parent_record_offset":229720,"ancestor_record_offset":229572,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":229812,"parent_record_offset":229784,"ancestor_record_offset":229572,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":230520,"parent_record_offset":229812,"ancestor_record_offset":229572,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1613,"record_offset":230172,"parent_record_offset":229812,"ancestor_record_offset":229572,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1613,"record_offset":229664,"parent_record_offset":229636,"ancestor_record_offset":229572,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1613,"record_offset":231392,"parent_record_offset":231304,"ancestor_record_offset":229572,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1613,"record_offset":231304,"parent_record_offset":231280,"ancestor_record_offset":229572,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1613,"record_offset":231416,"parent_record_offset":231392,"ancestor_record_offset":229572,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1613,"record_offset":230708,"parent_record_offset":230684,"ancestor_record_offset":229572,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1613,"record_offset":230756,"parent_record_offset":230732,"ancestor_record_offset":229572,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1613,"record_offset":230732,"parent_record_offset":230708,"ancestor_record_offset":229572,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1613,"record_offset":231544,"parent_record_offset":231512,"ancestor_record_offset":229572,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1613,"record_offset":231512,"parent_record_offset":231484,"ancestor_record_offset":229572,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1613,"record_offset":230684,"parent_record_offset":230660,"ancestor_record_offset":229572,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1613,"record_offset":230872,"parent_record_offset":230848,"ancestor_record_offset":229572,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1613,"record_offset":230848,"parent_record_offset":230824,"ancestor_record_offset":229572,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::save_thread::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 499
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12C04
// 首条位置: {"module":1686,"inlinee":76804}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":41524,"parent_record_offset":41500,"ancestor_record_offset":41500,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":41628,"parent_record_offset":41604,"ancestor_record_offset":41500,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":1686,"record_offset":41604,"parent_record_offset":41580,"ancestor_record_offset":41500,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$20::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":1686,"record_offset":41580,"parent_record_offset":41556,"ancestor_record_offset":41500,"inlinee":5093,"source_line":14,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$23::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":1686,"record_offset":41556,"parent_record_offset":41524,"ancestor_record_offset":41500,"inlinee":5091,"source_line":149,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::execute_open_file_cmd
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 670
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12C0F
// 首条位置: {"module":1686,"inlinee":76815}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1686,"record_offset":118692,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":103852,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":112200,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":111448,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":102932,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1686,"record_offset":102964,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$24::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1686,"record_offset":106412,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":11083,"source_line":280,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1686,"record_offset":106380,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1686,"record_offset":115424,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::flatten
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1686,"record_offset":106316,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":27668,"source_line":1514,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":111360,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":102732,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":102992,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":121116,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::inspect_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":104980,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":13626,"source_line":899,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$6::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":105356,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":32591,"source_line":1768,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_backup_list::SaveBackupList::into_files
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_backup_list.rs
//     evidence={"module":1686,"record_offset":115396,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":32631,"source_line":136,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::full_write_history_file
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":113284,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":76817,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":102848,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":104052,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":104020,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":103880,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":1686,"record_offset":118352,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":7841,"source_line":519,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":1686,"record_offset":118664,"parent_record_offset":102452,"ancestor_record_offset":102452,"inlinee":12559,"source_line":289,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":118552,"parent_record_offset":118528,"ancestor_record_offset":102452,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":118576,"parent_record_offset":118552,"ancestor_record_offset":102452,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":103412,"parent_record_offset":103384,"ancestor_record_offset":102452,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":118496,"parent_record_offset":118460,"ancestor_record_offset":102452,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":118528,"parent_record_offset":118496,"ancestor_record_offset":102452,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":103384,"parent_record_offset":103168,"ancestor_record_offset":102452,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1686,"record_offset":118460,"parent_record_offset":118424,"ancestor_record_offset":102452,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1686,"record_offset":105784,"parent_record_offset":105756,"ancestor_record_offset":102452,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":112280,"parent_record_offset":112248,"ancestor_record_offset":102452,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":118960,"parent_record_offset":118924,"ancestor_record_offset":102452,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":103568,"parent_record_offset":103540,"ancestor_record_offset":102452,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":103540,"parent_record_offset":103516,"ancestor_record_offset":102452,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":112328,"parent_record_offset":112304,"ancestor_record_offset":102452,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":112304,"parent_record_offset":112280,"ancestor_record_offset":102452,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":119036,"parent_record_offset":118996,"ancestor_record_offset":102452,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":118996,"parent_record_offset":118960,"ancestor_record_offset":102452,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":103516,"parent_record_offset":103492,"ancestor_record_offset":102452,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":118884,"parent_record_offset":118848,"ancestor_record_offset":102452,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":118848,"parent_record_offset":118812,"ancestor_record_offset":102452,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":118776,"parent_record_offset":118728,"ancestor_record_offset":102452,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":118812,"parent_record_offset":118776,"ancestor_record_offset":102452,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1686,"record_offset":118728,"parent_record_offset":118692,"ancestor_record_offset":102452,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1686,"record_offset":108056,"parent_record_offset":108020,"ancestor_record_offset":102452,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1686,"record_offset":108020,"parent_record_offset":107996,"ancestor_record_offset":102452,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":112248,"parent_record_offset":112200,"ancestor_record_offset":102452,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":118924,"parent_record_offset":118884,"ancestor_record_offset":102452,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1686,"record_offset":113596,"parent_record_offset":113568,"ancestor_record_offset":102452,"inlinee":12447,"source_line":316,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":113568,"parent_record_offset":113532,"ancestor_record_offset":102452,"inlinee":12446,"source_line":33,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":113532,"parent_record_offset":113284,"ancestor_record_offset":102452,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1686,"record_offset":105816,"parent_record_offset":105784,"ancestor_record_offset":102452,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":116016,"parent_record_offset":115984,"ancestor_record_offset":102452,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1686,"record_offset":116076,"parent_record_offset":116044,"ancestor_record_offset":102452,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":105140,"parent_record_offset":105032,"ancestor_record_offset":102452,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":111496,"parent_record_offset":111472,"ancestor_record_offset":102452,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":119192,"parent_record_offset":119168,"ancestor_record_offset":102452,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":116360,"parent_record_offset":116328,"ancestor_record_offset":102452,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":1686,"record_offset":116452,"parent_record_offset":116424,"ancestor_record_offset":102452,"inlinee":9901,"source_line":57,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::Flatten::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1686,"record_offset":106344,"parent_record_offset":106316,"ancestor_record_offset":102452,"inlinee":27670,"source_line":244,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1686,"record_offset":107752,"parent_record_offset":107672,"ancestor_record_offset":102452,"inlinee":11087,"source_line":807,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$24::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1686,"record_offset":107636,"parent_record_offset":107548,"ancestor_record_offset":102452,"inlinee":11083,"source_line":280,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1686,"record_offset":107672,"parent_record_offset":107636,"ancestor_record_offset":102452,"inlinee":11085,"source_line":573,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$59::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1686,"record_offset":106456,"parent_record_offset":106412,"ancestor_record_offset":102452,"inlinee":13694,"source_line":906,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1686,"record_offset":107548,"parent_record_offset":107500,"ancestor_record_offset":102452,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1686,"record_offset":107500,"parent_record_offset":106456,"ancestor_record_offset":102452,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1686,"record_offset":116424,"parent_record_offset":116396,"ancestor_record_offset":102452,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1686,"record_offset":116572,"parent_record_offset":116508,"ancestor_record_offset":102452,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1686,"record_offset":116396,"parent_record_offset":116268,"ancestor_record_offset":102452,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1686,"record_offset":116768,"parent_record_offset":116744,"ancestor_record_offset":102452,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":102756,"parent_record_offset":102732,"ancestor_record_offset":102452,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":105840,"parent_record_offset":105816,"ancestor_record_offset":102452,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":103288,"parent_record_offset":103260,"ancestor_record_offset":102452,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":103316,"parent_record_offset":103288,"ancestor_record_offset":102452,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":116836,"parent_record_offset":116508,"ancestor_record_offset":102452,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":103600,"parent_record_offset":103568,"ancestor_record_offset":102452,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":116044,"parent_record_offset":115984,"ancestor_record_offset":102452,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1686,"record_offset":116596,"parent_record_offset":116572,"ancestor_record_offset":102452,"inlinee":4196,"source_line":166,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1686,"record_offset":107828,"parent_record_offset":107752,"ancestor_record_offset":102452,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":107596,"parent_record_offset":107548,"ancestor_record_offset":102452,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":113504,"parent_record_offset":113284,"ancestor_record_offset":102452,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":109612,"parent_record_offset":109588,"ancestor_record_offset":102452,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":115736,"parent_record_offset":115708,"ancestor_record_offset":102452,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":111472,"parent_record_offset":111448,"ancestor_record_offset":102452,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":110328,"parent_record_offset":110252,"ancestor_record_offset":102452,"inlinee":6199,"source_line":2499,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":105228,"parent_record_offset":105196,"ancestor_record_offset":102452,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":105256,"parent_record_offset":105196,"ancestor_record_offset":102452,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1686,"record_offset":119168,"parent_record_offset":118884,"ancestor_record_offset":102452,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=159
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":103016,"parent_record_offset":102992,"ancestor_record_offset":102452,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":113468,"parent_record_offset":113444,"ancestor_record_offset":102452,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":115824,"parent_record_offset":115708,"ancestor_record_offset":102452,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":103352,"parent_record_offset":103200,"ancestor_record_offset":102452,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":105584,"parent_record_offset":105556,"ancestor_record_offset":102452,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":105632,"parent_record_offset":105608,"ancestor_record_offset":102452,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":105608,"parent_record_offset":105584,"ancestor_record_offset":102452,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":116540,"parent_record_offset":116508,"ancestor_record_offset":102452,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":108088,"parent_record_offset":108056,"ancestor_record_offset":102452,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":102784,"parent_record_offset":102756,"ancestor_record_offset":102452,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":118604,"parent_record_offset":118576,"ancestor_record_offset":102452,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":115848,"parent_record_offset":115824,"ancestor_record_offset":102452,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":102812,"parent_record_offset":102756,"ancestor_record_offset":102452,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":104260,"parent_record_offset":104052,"ancestor_record_offset":102452,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":105396,"parent_record_offset":105356,"ancestor_record_offset":102452,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":109588,"parent_record_offset":109536,"ancestor_record_offset":102452,"inlinee":32593,"source_line":1908,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$6::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":106520,"parent_record_offset":106456,"ancestor_record_offset":102452,"inlinee":32591,"source_line":1768,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":111568,"parent_record_offset":111544,"ancestor_record_offset":102452,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":111592,"parent_record_offset":111568,"ancestor_record_offset":102452,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1686,"record_offset":113420,"parent_record_offset":113284,"ancestor_record_offset":102452,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1686,"record_offset":113444,"parent_record_offset":113420,"ancestor_record_offset":102452,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1686,"record_offset":116508,"parent_record_offset":116480,"ancestor_record_offset":102452,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":111520,"parent_record_offset":111496,"ancestor_record_offset":102452,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":108568,"parent_record_offset":108532,"ancestor_record_offset":102452,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":102872,"parent_record_offset":102848,"ancestor_record_offset":102452,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":102896,"parent_record_offset":102872,"ancestor_record_offset":102452,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":108116,"parent_record_offset":108056,"ancestor_record_offset":102452,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":108148,"parent_record_offset":108020,"ancestor_record_offset":102452,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_thread::execute_open_file_cmd::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":105032,"parent_record_offset":104980,"ancestor_record_offset":102452,"inlinee":76816,"source_line":692,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":116300,"parent_record_offset":116268,"ancestor_record_offset":102452,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":116268,"parent_record_offset":116240,"ancestor_record_offset":102452,"inlinee":6915,"source_line":4480,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":116792,"parent_record_offset":116768,"ancestor_record_offset":102452,"inlinee":6918,"source_line":4492,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":116328,"parent_record_offset":116300,"ancestor_record_offset":102452,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":103200,"parent_record_offset":103168,"ancestor_record_offset":102452,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":103232,"parent_record_offset":103200,"ancestor_record_offset":102452,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":103112,"parent_record_offset":103088,"ancestor_record_offset":102452,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":103168,"parent_record_offset":103112,"ancestor_record_offset":102452,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":103140,"parent_record_offset":103112,"ancestor_record_offset":102452,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":103260,"parent_record_offset":103200,"ancestor_record_offset":102452,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":103088,"parent_record_offset":103064,"ancestor_record_offset":102452,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1686,"record_offset":116240,"parent_record_offset":116212,"ancestor_record_offset":102452,"inlinee":6932,"source_line":1307,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":105284,"parent_record_offset":105196,"ancestor_record_offset":102452,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":105172,"parent_record_offset":105032,"ancestor_record_offset":102452,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":105196,"parent_record_offset":105172,"ancestor_record_offset":102452,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":105308,"parent_record_offset":105284,"ancestor_record_offset":102452,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":105056,"parent_record_offset":105032,"ancestor_record_offset":102452,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":116184,"parent_record_offset":115464,"ancestor_record_offset":102452,"inlinee":13714,"source_line":392,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$7::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":115464,"parent_record_offset":115424,"ancestor_record_offset":102452,"inlinee":13713,"source_line":1067,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":116212,"parent_record_offset":115464,"ancestor_record_offset":102452,"inlinee":13716,"source_line":1117,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":111412,"parent_record_offset":111388,"ancestor_record_offset":102452,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":116648,"parent_record_offset":116620,"ancestor_record_offset":102452,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":103928,"parent_record_offset":103904,"ancestor_record_offset":102452,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":103976,"parent_record_offset":103952,"ancestor_record_offset":102452,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":103952,"parent_record_offset":103928,"ancestor_record_offset":102452,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1686,"record_offset":115536,"parent_record_offset":115512,"ancestor_record_offset":102452,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1686,"record_offset":115784,"parent_record_offset":115760,"ancestor_record_offset":102452,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1686,"record_offset":115648,"parent_record_offset":115620,"ancestor_record_offset":102452,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1686,"record_offset":115984,"parent_record_offset":115584,"ancestor_record_offset":102452,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1686,"record_offset":115512,"parent_record_offset":115464,"ancestor_record_offset":102452,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::into_inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":113644,"parent_record_offset":113284,"ancestor_record_offset":102452,"inlinee":76818,"source_line":116,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":113392,"parent_record_offset":113284,"ancestor_record_offset":102452,"inlinee":21628,"source_line":98,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":105556,"parent_record_offset":105524,"ancestor_record_offset":102452,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":105524,"parent_record_offset":105496,"ancestor_record_offset":102452,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":1686,"record_offset":119404,"parent_record_offset":119380,"ancestor_record_offset":102452,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":103904,"parent_record_offset":103880,"ancestor_record_offset":102452,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":116620,"parent_record_offset":116596,"ancestor_record_offset":102452,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":104116,"parent_record_offset":104092,"ancestor_record_offset":102452,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":104092,"parent_record_offset":104052,"ancestor_record_offset":102452,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":1686,"record_offset":118388,"parent_record_offset":118352,"ancestor_record_offset":102452,"inlinee":7806,"source_line":197,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\counter.rs
//     evidence={"module":1686,"record_offset":118424,"parent_record_offset":118388,"ancestor_record_offset":102452,"inlinee":7808,"source_line":20,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1686,"record_offset":116676,"parent_record_offset":116648,"ancestor_record_offset":102452,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1686,"record_offset":115676,"parent_record_offset":115648,"ancestor_record_offset":102452,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1686,"record_offset":115708,"parent_record_offset":115676,"ancestor_record_offset":102452,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":1686,"record_offset":116700,"parent_record_offset":116676,"ancestor_record_offset":102452,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1686,"record_offset":115584,"parent_record_offset":115560,"ancestor_record_offset":102452,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1686,"record_offset":115560,"parent_record_offset":115536,"ancestor_record_offset":102452,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::execute_open_file_cmd::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 692
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12C10
// 首条位置: {"module":1686,"inlinee":76816}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":105140,"parent_record_offset":105032,"ancestor_record_offset":105032,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":105172,"parent_record_offset":105032,"ancestor_record_offset":105032,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":105056,"parent_record_offset":105032,"ancestor_record_offset":105032,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":105228,"parent_record_offset":105196,"ancestor_record_offset":105032,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":105256,"parent_record_offset":105196,"ancestor_record_offset":105032,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":105080,"parent_record_offset":105056,"ancestor_record_offset":105032,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":105104,"parent_record_offset":105080,"ancestor_record_offset":105032,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":105284,"parent_record_offset":105196,"ancestor_record_offset":105032,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":105196,"parent_record_offset":105172,"ancestor_record_offset":105032,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":105308,"parent_record_offset":105284,"ancestor_record_offset":105032,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::execute_open_file_cmd::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 731
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7045
// 首条位置: {"module":506,"inlinee":28741}
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
// IPI IDs: 0x12C11
// 首条位置: {"module":1686,"inlinee":76817}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":113532,"parent_record_offset":113284,"ancestor_record_offset":113284,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":113504,"parent_record_offset":113284,"ancestor_record_offset":113284,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":113812,"parent_record_offset":113284,"ancestor_record_offset":113284,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1686,"record_offset":113420,"parent_record_offset":113284,"ancestor_record_offset":113284,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::into_inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":113644,"parent_record_offset":113284,"ancestor_record_offset":113284,"inlinee":76818,"source_line":116,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1686,"record_offset":113392,"parent_record_offset":113284,"ancestor_record_offset":113284,"inlinee":21628,"source_line":98,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":113672,"parent_record_offset":113284,"ancestor_record_offset":113284,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":114060,"parent_record_offset":114036,"ancestor_record_offset":113284,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":114036,"parent_record_offset":113980,"ancestor_record_offset":113284,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":114008,"parent_record_offset":113980,"ancestor_record_offset":113284,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":113980,"parent_record_offset":113956,"ancestor_record_offset":113284,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":113956,"parent_record_offset":113932,"ancestor_record_offset":113284,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1686,"record_offset":113596,"parent_record_offset":113568,"ancestor_record_offset":113284,"inlinee":12447,"source_line":316,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":113568,"parent_record_offset":113532,"ancestor_record_offset":113284,"inlinee":12446,"source_line":33,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":113836,"parent_record_offset":113812,"ancestor_record_offset":113284,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":113468,"parent_record_offset":113444,"ancestor_record_offset":113284,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1686,"record_offset":113444,"parent_record_offset":113420,"ancestor_record_offset":113284,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":113720,"parent_record_offset":113696,"ancestor_record_offset":113284,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":113768,"parent_record_offset":113744,"ancestor_record_offset":113284,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":113744,"parent_record_offset":113720,"ancestor_record_offset":113284,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":113696,"parent_record_offset":113672,"ancestor_record_offset":113284,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::capture_snapshot
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 793
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12C0E
// 首条位置: {"module":1686,"inlinee":76814}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":97452,"parent_record_offset":96316,"ancestor_record_offset":96316,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":96776,"parent_record_offset":96316,"ancestor_record_offset":96316,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":96432,"parent_record_offset":96316,"ancestor_record_offset":96316,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":97608,"parent_record_offset":96316,"ancestor_record_offset":96316,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::HistorySnapshotId::file_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1686,"record_offset":96664,"parent_record_offset":96316,"ancestor_record_offset":96316,"inlinee":31761,"source_line":283,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::HistorySnapshotId::from_snapshot_blob
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1686,"record_offset":96464,"parent_record_offset":96316,"ancestor_record_offset":96316,"inlinee":31760,"source_line":279,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":97172,"parent_record_offset":96316,"ancestor_record_offset":96316,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":96632,"parent_record_offset":96316,"ancestor_record_offset":96316,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":96492,"parent_record_offset":96316,"ancestor_record_offset":96316,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":96976,"parent_record_offset":96952,"ancestor_record_offset":96316,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":96952,"parent_record_offset":96896,"ancestor_record_offset":96316,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1686,"record_offset":96688,"parent_record_offset":96664,"ancestor_record_offset":96316,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1686,"record_offset":96736,"parent_record_offset":96712,"ancestor_record_offset":96316,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":96924,"parent_record_offset":96896,"ancestor_record_offset":96316,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":96896,"parent_record_offset":96872,"ancestor_record_offset":96316,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":96872,"parent_record_offset":96848,"ancestor_record_offset":96316,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1686,"record_offset":97532,"parent_record_offset":97488,"ancestor_record_offset":96316,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1686,"record_offset":97488,"parent_record_offset":97452,"ancestor_record_offset":96316,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1686,"record_offset":97224,"parent_record_offset":97172,"ancestor_record_offset":96316,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":96712,"parent_record_offset":96688,"ancestor_record_offset":96316,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=52
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":96800,"parent_record_offset":96776,"ancestor_record_offset":96316,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":96540,"parent_record_offset":96516,"ancestor_record_offset":96316,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":96588,"parent_record_offset":96564,"ancestor_record_offset":96316,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":96564,"parent_record_offset":96540,"ancestor_record_offset":96316,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":97420,"parent_record_offset":97172,"ancestor_record_offset":96316,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":97196,"parent_record_offset":97172,"ancestor_record_offset":96316,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":1686,"record_offset":98976,"parent_record_offset":98952,"ancestor_record_offset":96316,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":96516,"parent_record_offset":96492,"ancestor_record_offset":96316,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":97272,"parent_record_offset":97248,"ancestor_record_offset":96316,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":97248,"parent_record_offset":97224,"ancestor_record_offset":96316,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::snapshot_gc_thread
// 记录形式: compiled_procedure; instances=1
// 源码行: 817
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1686,"record_offset":132456,"rva":24294016,"va":5393003136}
// 编译实例 RVAs: 0x172b280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-10359266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e81eee61ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5393003789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140149f50
//     owner=unresolved_namespace; count=3
//     target_aliases=35 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22946813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e803dca1fe","size":5,"soft_fail":false,"target_va":5370060624,"va":5393007432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18328988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e86452e8fe","size":5,"soft_fail":false,"target_va":5374676336,"va":5393005319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d8fc0
//     owner=unresolved_namespace; count=2
//     target_aliases=23 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18167828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8ecc7eafe","size":5,"soft_fail":false,"target_va":5374840768,"va":5393008591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dd0b0
//     owner=unresolved_namespace; count=2
//     target_aliases=13 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18149949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8c30debfe","size":5,"soft_fail":false,"target_va":5374857392,"va":5393007336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=3
//     target_aliases=92 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18103726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e852c2ebfe","size":5,"soft_fail":false,"target_va":5374900736,"va":5393004457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18103454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e862c3ebfe","size":5,"soft_fail":false,"target_va":5374904480,"va":5393007929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17922457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e86786eefe","size":5,"soft_fail":false,"target_va":5375081488,"va":5393003940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=27
//     target_aliases=1 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17922393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8a786eefe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393003892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=6
//     target_aliases=69 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17487584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e82029f5fe","size":5,"soft_fail":false,"target_va":5375516032,"va":5393003611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408778c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15419223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8a9b814ff","size":5,"soft_fail":false,"target_va":5377587392,"va":5393006610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408da040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15013816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e848e81aff","size":5,"soft_fail":false,"target_va":5377990720,"va":5393004531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10300165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8fbd462ff","size":5,"soft_fail":false,"target_va":5382704752,"va":5393004912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=6
//     target_aliases=5 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10175755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8f5ba64ff","size":5,"soft_fail":false,"target_va":5382827728,"va":5393003478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141286730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4869819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e845b1b5ff","size":5,"soft_fail":false,"target_va":5388134192,"va":5393004006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141523680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_thread.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2130211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8dd7edfff","size":5,"soft_fail":false,"target_va":5390874240,"va":5393004446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18533768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e888cd1a01","size":5,"soft_fail":false,"target_va":5411540963,"va":5393007190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10294832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8d0e962ff","size":5,"soft_fail":false,"target_va":5382708672,"va":5393003499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10260923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8456e63ff","size":5,"soft_fail":false,"target_va":5382743968,"va":5393004886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10262051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8dd6963ff","size":5,"soft_fail":false,"target_va":5382741504,"va":5393003550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_stem
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10261213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8236d63ff","size":5,"soft_fail":false,"target_va":5382743808,"va":5393005016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::unlink
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10233640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8d8d863ff","size":5,"soft_fail":false,"target_va":5382772112,"va":5393005747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10257928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8f87963ff","size":5,"soft_fail":false,"target_va":5382748432,"va":5393006355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10255621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393003136,"raw_hex":"e8fb8263ff","size":5,"soft_fail":false,"target_va":5382747616,"va":5393003232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5393003136,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5393003863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":132652,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":132708,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":133276,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1686,"record_offset":135808,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":133624,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":135948,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":8084,"source_line":607,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::save_thread::nuke_unreferenced_shapshots
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_thread.rs
//     evidence={"module":1686,"record_offset":136720,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":76819,"source_line":868,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":133448,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":133192,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":135864,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":13716,"source_line":1117,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":132880,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::io::buffered::bufreader::BufReader::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":1686,"record_offset":134424,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":18036,"source_line":73,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":133308,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":1686,"record_offset":135920,"parent_record_offset":132456,"ancestor_record_offset":132456,"inlinee":7835,"source_line":607,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":134664,"parent_record_offset":134640,"ancestor_record_offset":132456,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":134688,"parent_record_offset":134664,"ancestor_record_offset":132456,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":134048,"parent_record_offset":134024,"ancestor_record_offset":132456,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":134640,"parent_record_offset":134612,"ancestor_record_offset":132456,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":134024,"parent_record_offset":133936,"ancestor_record_offset":132456,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$2::new_uninit_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1686,"record_offset":134524,"parent_record_offset":134496,"ancestor_record_offset":132456,"inlinee":5319,"source_line":663,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1686,"record_offset":133936,"parent_record_offset":133908,"ancestor_record_offset":132456,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":134552,"parent_record_offset":134524,"ancestor_record_offset":132456,"inlinee":5320,"source_line":123,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":136312,"parent_record_offset":136280,"ancestor_record_offset":132456,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":136280,"parent_record_offset":136252,"ancestor_record_offset":132456,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":134612,"parent_record_offset":134580,"ancestor_record_offset":132456,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":134580,"parent_record_offset":134552,"ancestor_record_offset":132456,"inlinee":5321,"source_line":145,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":136252,"parent_record_offset":136224,"ancestor_record_offset":132456,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":136860,"parent_record_offset":136720,"ancestor_record_offset":132456,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":145568,"parent_record_offset":145544,"ancestor_record_offset":132456,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":144404,"parent_record_offset":144376,"ancestor_record_offset":132456,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":136028,"parent_record_offset":136004,"ancestor_record_offset":132456,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":136460,"parent_record_offset":136436,"ancestor_record_offset":132456,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":136916,"parent_record_offset":136720,"ancestor_record_offset":132456,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1686,"record_offset":133968,"parent_record_offset":133936,"ancestor_record_offset":132456,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::to_digit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":1686,"record_offset":138628,"parent_record_offset":138452,"ancestor_record_offset":132456,"inlinee":9730,"source_line":390,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":137308,"parent_record_offset":136720,"ancestor_record_offset":132456,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":139792,"parent_record_offset":136720,"ancestor_record_offset":132456,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":137356,"parent_record_offset":137332,"ancestor_record_offset":132456,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1686,"record_offset":132964,"parent_record_offset":132880,"ancestor_record_offset":132456,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1686,"record_offset":139112,"parent_record_offset":139088,"ancestor_record_offset":132456,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1686,"record_offset":139196,"parent_record_offset":139172,"ancestor_record_offset":132456,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":139136,"parent_record_offset":139112,"ancestor_record_offset":132456,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":139528,"parent_record_offset":138976,"ancestor_record_offset":132456,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1686,"record_offset":135832,"parent_record_offset":135808,"ancestor_record_offset":132456,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1686,"record_offset":144428,"parent_record_offset":144404,"ancestor_record_offset":132456,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":133992,"parent_record_offset":133968,"ancestor_record_offset":132456,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::can_not_overflow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1686,"record_offset":138540,"parent_record_offset":138452,"ancestor_record_offset":132456,"inlinee":9727,"source_line":1332,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":142904,"parent_record_offset":142876,"ancestor_record_offset":132456,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":136604,"parent_record_offset":136572,"ancestor_record_offset":132456,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$39::from_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1686,"record_offset":138404,"parent_record_offset":138380,"ancestor_record_offset":132456,"inlinee":9721,"source_line":1385,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$40::from_ascii_radix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1686,"record_offset":138452,"parent_record_offset":138428,"ancestor_record_offset":132456,"inlinee":9724,"source_line":1510,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$40::from_str_radix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1686,"record_offset":138428,"parent_record_offset":138404,"ancestor_record_offset":132456,"inlinee":9723,"source_line":1429,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":138656,"parent_record_offset":138628,"ancestor_record_offset":132456,"inlinee":4121,"source_line":2021,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":138688,"parent_record_offset":138452,"ancestor_record_offset":132456,"inlinee":7229,"source_line":486,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":138572,"parent_record_offset":138452,"ancestor_record_offset":132456,"inlinee":9728,"source_line":870,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":138596,"parent_record_offset":138572,"ancestor_record_offset":132456,"inlinee":9729,"source_line":2579,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1686,"record_offset":139344,"parent_record_offset":139316,"ancestor_record_offset":132456,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1686,"record_offset":139316,"parent_record_offset":139288,"ancestor_record_offset":132456,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1686,"record_offset":137248,"parent_record_offset":137220,"ancestor_record_offset":132456,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":137220,"parent_record_offset":136720,"ancestor_record_offset":132456,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":137332,"parent_record_offset":137308,"ancestor_record_offset":132456,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":133504,"parent_record_offset":133472,"ancestor_record_offset":132456,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":133532,"parent_record_offset":133472,"ancestor_record_offset":132456,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=105
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":133652,"parent_record_offset":133624,"ancestor_record_offset":132456,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":132676,"parent_record_offset":132652,"ancestor_record_offset":132456,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":142936,"parent_record_offset":142844,"ancestor_record_offset":132456,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":133736,"parent_record_offset":133708,"ancestor_record_offset":132456,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":133784,"parent_record_offset":133760,"ancestor_record_offset":132456,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":133760,"parent_record_offset":133736,"ancestor_record_offset":132456,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":132792,"parent_record_offset":132708,"ancestor_record_offset":132456,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":132820,"parent_record_offset":132708,"ancestor_record_offset":132456,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":132756,"parent_record_offset":132708,"ancestor_record_offset":132456,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":132844,"parent_record_offset":132820,"ancestor_record_offset":132456,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1686,"record_offset":135972,"parent_record_offset":135948,"ancestor_record_offset":132456,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":137428,"parent_record_offset":137404,"ancestor_record_offset":132456,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":137452,"parent_record_offset":137428,"ancestor_record_offset":132456,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::parse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1686,"record_offset":138380,"parent_record_offset":138356,"ancestor_record_offset":132456,"inlinee":5731,"source_line":2607,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":137380,"parent_record_offset":137356,"ancestor_record_offset":132456,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":133216,"parent_record_offset":133192,"ancestor_record_offset":132456,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":133240,"parent_record_offset":133216,"ancestor_record_offset":132456,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::HistorySnapshotId::from_file_stem
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1686,"record_offset":138356,"parent_record_offset":136720,"ancestor_record_offset":132456,"inlinee":31762,"source_line":288,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$44::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1686,"record_offset":139484,"parent_record_offset":139460,"ancestor_record_offset":132456,"inlinee":33400,"source_line":271,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1686,"record_offset":139256,"parent_record_offset":139228,"ancestor_record_offset":132456,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1686,"record_offset":139288,"parent_record_offset":139256,"ancestor_record_offset":132456,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1686,"record_offset":139228,"parent_record_offset":138976,"ancestor_record_offset":132456,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1686,"record_offset":139088,"parent_record_offset":138976,"ancestor_record_offset":132456,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1686,"record_offset":139556,"parent_record_offset":138976,"ancestor_record_offset":132456,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1686,"record_offset":139172,"parent_record_offset":138976,"ancestor_record_offset":132456,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1686,"record_offset":139060,"parent_record_offset":138976,"ancestor_record_offset":132456,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1686,"record_offset":139436,"parent_record_offset":139412,"ancestor_record_offset":132456,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":138844,"parent_record_offset":138820,"ancestor_record_offset":132456,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":138868,"parent_record_offset":138844,"ancestor_record_offset":132456,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":139412,"parent_record_offset":139388,"ancestor_record_offset":132456,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":138900,"parent_record_offset":138868,"ancestor_record_offset":132456,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":139648,"parent_record_offset":138976,"ancestor_record_offset":132456,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":138952,"parent_record_offset":138928,"ancestor_record_offset":132456,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":138928,"parent_record_offset":138868,"ancestor_record_offset":132456,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":142844,"parent_record_offset":142812,"ancestor_record_offset":132456,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":142756,"parent_record_offset":142732,"ancestor_record_offset":132456,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":138976,"parent_record_offset":138952,"ancestor_record_offset":132456,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":142812,"parent_record_offset":142756,"ancestor_record_offset":132456,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":142784,"parent_record_offset":142756,"ancestor_record_offset":132456,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":139032,"parent_record_offset":138976,"ancestor_record_offset":132456,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":142876,"parent_record_offset":142844,"ancestor_record_offset":132456,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":142732,"parent_record_offset":142708,"ancestor_record_offset":132456,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":139388,"parent_record_offset":138976,"ancestor_record_offset":132456,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1686,"record_offset":138820,"parent_record_offset":138796,"ancestor_record_offset":132456,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1686,"record_offset":135888,"parent_record_offset":135864,"ancestor_record_offset":132456,"inlinee":6932,"source_line":1307,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":133560,"parent_record_offset":133472,"ancestor_record_offset":132456,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":139824,"parent_record_offset":136720,"ancestor_record_offset":132456,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":133472,"parent_record_offset":133448,"ancestor_record_offset":132456,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":133584,"parent_record_offset":133560,"ancestor_record_offset":132456,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":139708,"parent_record_offset":136720,"ancestor_record_offset":132456,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":138796,"parent_record_offset":136720,"ancestor_record_offset":132456,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":137272,"parent_record_offset":137248,"ancestor_record_offset":132456,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":133060,"parent_record_offset":133036,"ancestor_record_offset":132456,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":133108,"parent_record_offset":133084,"ancestor_record_offset":132456,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":133084,"parent_record_offset":133060,"ancestor_record_offset":132456,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":132904,"parent_record_offset":132880,"ancestor_record_offset":132456,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":133160,"parent_record_offset":132880,"ancestor_record_offset":132456,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":140000,"parent_record_offset":136720,"ancestor_record_offset":132456,"inlinee":8068,"source_line":2342,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::BufReader::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":1686,"record_offset":134460,"parent_record_offset":134424,"ancestor_record_offset":132456,"inlinee":5316,"source_line":102,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":1686,"record_offset":134496,"parent_record_offset":134460,"ancestor_record_offset":132456,"inlinee":5317,"source_line":34,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":133708,"parent_record_offset":133680,"ancestor_record_offset":132456,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":133680,"parent_record_offset":133652,"ancestor_record_offset":132456,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":1686,"record_offset":134840,"parent_record_offset":134816,"ancestor_record_offset":132456,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":133036,"parent_record_offset":133012,"ancestor_record_offset":132456,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":133012,"parent_record_offset":132988,"ancestor_record_offset":132456,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":132988,"parent_record_offset":132964,"ancestor_record_offset":132456,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1686,"record_offset":132932,"parent_record_offset":132904,"ancestor_record_offset":132456,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_thread::nuke_unreferenced_shapshots
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 868
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12C13
// 首条位置: {"module":1686,"inlinee":76819}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1686,"record_offset":136860,"parent_record_offset":136720,"ancestor_record_offset":136720,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":136916,"parent_record_offset":136720,"ancestor_record_offset":136720,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":137308,"parent_record_offset":136720,"ancestor_record_offset":136720,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":139792,"parent_record_offset":136720,"ancestor_record_offset":136720,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":137220,"parent_record_offset":136720,"ancestor_record_offset":136720,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":137504,"parent_record_offset":136720,"ancestor_record_offset":136720,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::HistorySnapshotId::from_file_stem
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1686,"record_offset":138356,"parent_record_offset":136720,"ancestor_record_offset":136720,"inlinee":31762,"source_line":288,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":139824,"parent_record_offset":136720,"ancestor_record_offset":136720,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":139708,"parent_record_offset":136720,"ancestor_record_offset":136720,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1686,"record_offset":138796,"parent_record_offset":136720,"ancestor_record_offset":136720,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1686,"record_offset":140000,"parent_record_offset":136720,"ancestor_record_offset":136720,"inlinee":8068,"source_line":2342,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":137080,"parent_record_offset":136720,"ancestor_record_offset":136720,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":137752,"parent_record_offset":137728,"ancestor_record_offset":136720,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1686,"record_offset":137728,"parent_record_offset":137672,"ancestor_record_offset":136720,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1686,"record_offset":141432,"parent_record_offset":141404,"ancestor_record_offset":136720,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":137700,"parent_record_offset":137672,"ancestor_record_offset":136720,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":137672,"parent_record_offset":137648,"ancestor_record_offset":136720,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1686,"record_offset":137648,"parent_record_offset":137624,"ancestor_record_offset":136720,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":141944,"parent_record_offset":141920,"ancestor_record_offset":136720,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1686,"record_offset":142376,"parent_record_offset":142352,"ancestor_record_offset":136720,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1686,"record_offset":141464,"parent_record_offset":141432,"ancestor_record_offset":136720,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::to_digit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":1686,"record_offset":138628,"parent_record_offset":138452,"ancestor_record_offset":136720,"inlinee":9730,"source_line":390,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1686,"record_offset":137356,"parent_record_offset":137332,"ancestor_record_offset":136720,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1686,"record_offset":139112,"parent_record_offset":139088,"ancestor_record_offset":136720,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1686,"record_offset":139196,"parent_record_offset":139172,"ancestor_record_offset":136720,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":139136,"parent_record_offset":139112,"ancestor_record_offset":136720,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1686,"record_offset":139528,"parent_record_offset":138976,"ancestor_record_offset":136720,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1686,"record_offset":141488,"parent_record_offset":141464,"ancestor_record_offset":136720,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::can_not_overflow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1686,"record_offset":138540,"parent_record_offset":138452,"ancestor_record_offset":136720,"inlinee":9727,"source_line":1332,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":142904,"parent_record_offset":142876,"ancestor_record_offset":136720,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":142520,"parent_record_offset":142488,"ancestor_record_offset":136720,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$39::from_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1686,"record_offset":138404,"parent_record_offset":138380,"ancestor_record_offset":136720,"inlinee":9721,"source_line":1385,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$40::from_ascii_radix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1686,"record_offset":138452,"parent_record_offset":138428,"ancestor_record_offset":136720,"inlinee":9724,"source_line":1510,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$40::from_str_radix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1686,"record_offset":138428,"parent_record_offset":138404,"ancestor_record_offset":136720,"inlinee":9723,"source_line":1429,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":138656,"parent_record_offset":138628,"ancestor_record_offset":136720,"inlinee":4121,"source_line":2021,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":138688,"parent_record_offset":138452,"ancestor_record_offset":136720,"inlinee":7229,"source_line":486,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":138572,"parent_record_offset":138452,"ancestor_record_offset":136720,"inlinee":9728,"source_line":870,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1686,"record_offset":138596,"parent_record_offset":138572,"ancestor_record_offset":136720,"inlinee":9729,"source_line":2579,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1686,"record_offset":139344,"parent_record_offset":139316,"ancestor_record_offset":136720,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1686,"record_offset":139316,"parent_record_offset":139288,"ancestor_record_offset":136720,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1686,"record_offset":137248,"parent_record_offset":137220,"ancestor_record_offset":136720,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1686,"record_offset":137332,"parent_record_offset":137308,"ancestor_record_offset":136720,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":139880,"parent_record_offset":139848,"ancestor_record_offset":136720,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1686,"record_offset":139908,"parent_record_offset":139848,"ancestor_record_offset":136720,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":137528,"parent_record_offset":137504,"ancestor_record_offset":136720,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":136884,"parent_record_offset":136860,"ancestor_record_offset":136720,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":142936,"parent_record_offset":142844,"ancestor_record_offset":136720,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":141232,"parent_record_offset":141204,"ancestor_record_offset":136720,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":141280,"parent_record_offset":141256,"ancestor_record_offset":136720,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1686,"record_offset":141256,"parent_record_offset":141232,"ancestor_record_offset":136720,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":136992,"parent_record_offset":136916,"ancestor_record_offset":136720,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":137020,"parent_record_offset":136916,"ancestor_record_offset":136720,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1686,"record_offset":136960,"parent_record_offset":136916,"ancestor_record_offset":136720,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1686,"record_offset":137044,"parent_record_offset":137020,"ancestor_record_offset":136720,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":137428,"parent_record_offset":137404,"ancestor_record_offset":136720,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1686,"record_offset":137452,"parent_record_offset":137428,"ancestor_record_offset":136720,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::parse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1686,"record_offset":138380,"parent_record_offset":138356,"ancestor_record_offset":136720,"inlinee":5731,"source_line":2607,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1686,"record_offset":137380,"parent_record_offset":137356,"ancestor_record_offset":136720,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":139732,"parent_record_offset":139708,"ancestor_record_offset":136720,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1686,"record_offset":139756,"parent_record_offset":139732,"ancestor_record_offset":136720,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$44::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1686,"record_offset":139484,"parent_record_offset":139460,"ancestor_record_offset":136720,"inlinee":33400,"source_line":271,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1686,"record_offset":139256,"parent_record_offset":139228,"ancestor_record_offset":136720,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1686,"record_offset":139288,"parent_record_offset":139256,"ancestor_record_offset":136720,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1686,"record_offset":139228,"parent_record_offset":138976,"ancestor_record_offset":136720,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1686,"record_offset":139088,"parent_record_offset":138976,"ancestor_record_offset":136720,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1686,"record_offset":139556,"parent_record_offset":138976,"ancestor_record_offset":136720,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1686,"record_offset":139172,"parent_record_offset":138976,"ancestor_record_offset":136720,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1686,"record_offset":139060,"parent_record_offset":138976,"ancestor_record_offset":136720,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1686,"record_offset":139436,"parent_record_offset":139412,"ancestor_record_offset":136720,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":138844,"parent_record_offset":138820,"ancestor_record_offset":136720,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":138868,"parent_record_offset":138844,"ancestor_record_offset":136720,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":139412,"parent_record_offset":139388,"ancestor_record_offset":136720,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1686,"record_offset":138900,"parent_record_offset":138868,"ancestor_record_offset":136720,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":139648,"parent_record_offset":138976,"ancestor_record_offset":136720,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":138952,"parent_record_offset":138928,"ancestor_record_offset":136720,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":138928,"parent_record_offset":138868,"ancestor_record_offset":136720,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":142844,"parent_record_offset":142812,"ancestor_record_offset":136720,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":142756,"parent_record_offset":142732,"ancestor_record_offset":136720,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":138976,"parent_record_offset":138952,"ancestor_record_offset":136720,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":142812,"parent_record_offset":142756,"ancestor_record_offset":136720,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":142784,"parent_record_offset":142756,"ancestor_record_offset":136720,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":139032,"parent_record_offset":138976,"ancestor_record_offset":136720,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":142876,"parent_record_offset":142844,"ancestor_record_offset":136720,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":142732,"parent_record_offset":142708,"ancestor_record_offset":136720,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1686,"record_offset":139388,"parent_record_offset":138976,"ancestor_record_offset":136720,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1686,"record_offset":138820,"parent_record_offset":138796,"ancestor_record_offset":136720,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":139936,"parent_record_offset":139848,"ancestor_record_offset":136720,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1686,"record_offset":139848,"parent_record_offset":139824,"ancestor_record_offset":136720,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1686,"record_offset":139960,"parent_record_offset":139936,"ancestor_record_offset":136720,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":137272,"parent_record_offset":137248,"ancestor_record_offset":136720,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":137128,"parent_record_offset":137104,"ancestor_record_offset":136720,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":137176,"parent_record_offset":137152,"ancestor_record_offset":136720,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1686,"record_offset":137152,"parent_record_offset":137128,"ancestor_record_offset":136720,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":141204,"parent_record_offset":141176,"ancestor_record_offset":136720,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1686,"record_offset":141176,"parent_record_offset":141148,"ancestor_record_offset":136720,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":137104,"parent_record_offset":137080,"ancestor_record_offset":136720,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":140056,"parent_record_offset":140032,"ancestor_record_offset":136720,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01686.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1686,"record_offset":140032,"parent_record_offset":140000,"ancestor_record_offset":136720,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01686.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::save::save_thread::AutoSaveError>", "constructor": "enum2$", "arguments": ["country_core::systems::save::save_thread::AutoSaveError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >", "constructor": "tuple$", "arguments": ["std::path::PathBuf", "enum2$<country_core::systems::save::save_thread::AutoSaveError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$11::clone<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >,alloc::alloc::Global>", "constructor": "alloc::vec::impl$11::clone", "arguments": ["tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::save_thread::AutosaveErrorState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 37
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$20::drop::closure_env$2<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::impl$20::drop::closure_env$2", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<country_core::systems::save::save_thread::AutosaveErrorState>,std::sync::mpmc::impl$20::drop::closure_env$2<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<country_core::systems::save::save_thread::AutosaveErrorState>", "std::sync::mpmc::impl$20::drop::closure_env$2<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::disconnect<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::Channel::disconnect", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<country_core::systems::save::save_thread::AutosaveErrorState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<country_core::systems::save::save_thread::AutosaveErrorState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpsc::Receiver", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,std::sync::mpsc::Receiver<country_core::systems::save::save_thread::AutosaveErrorState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<country_core::systems::save::save_thread::AutosaveErrorState>,std::sync::mpmc::impl$6::drop::closure_env$2<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<country_core::systems::save::save_thread::AutosaveErrorState>", "std::sync::mpmc::impl$6::drop::closure_env$2<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::try_recv<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::list::Channel::try_recv", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::try_recv<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::array::Channel::try_recv", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::try_recv<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::Channel::try_recv", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState>", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["country_core::systems::save::save_thread::AutosaveErrorState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState> >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState> > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<country_core::systems::save::save_thread::AutosaveErrorState> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "array$<enum2$<core::option::Option<country_core::systems::save::save_thread::DeferredSection> >,10>", "constructor": "array$", "arguments": ["enum2$<core::option::Option<country_core::systems::save::save_thread::DeferredSection> >", "10"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<array$<enum2$<core::option::Option<country_core::systems::save::save_thread::DeferredSection> >,10> >", "constructor": "core::ptr::drop_in_place", "arguments": ["array$<enum2$<core::option::Option<country_core::systems::save::save_thread::DeferredSection> >,10>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::save_thread::SnapshotGcCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 19
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>", "constructor": "enum2$", "arguments": ["country_core::systems::save::save_thread::SnapshotGcCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::impl$20::drop::closure_env$2", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >,std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::try_recv<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::list::Channel::try_recv", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::send::closure$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::array::impl$1::send::closure$0", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::try_recv<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::array::Channel::try_recv", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::try_recv<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::zero::Channel::try_recv", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<country_core::systems::save::save_thread::SnapshotGcCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<country_core::systems::save::save_thread::SnapshotGcCmd> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 12; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell)
// 候选参数类型: ["country_core::render::pass_builder::draw_builder::DrawBuilder", "ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>", "bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<std::path::PathBuf>,ref$<std::path::PathBuf>)
// 候选参数类型: ["ref$<std::path::PathBuf>", "ref$<std::path::PathBuf>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::AutosaveErrorState::is_ok
// 信息 ID: 7dbec6369e9f7484d1903119
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$12::render @ 0x68059c -> 0xb92210 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::save_error::ui @ 0x1ec0d1d -> 0xb92210 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1686, inline_record=35660; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::CurrentSaveFile::backup_save
// 信息 ID: 41b5d9befd43e398b5fe3453
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=56540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::CurrentSaveFile::change_section
// 信息 ID: a00598321daca2478b3d37f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x171fab0 -> 0x171ece0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::save_thread @ 0x1721180 -> 0x171ece0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync
// 信息 ID: bdbc5a8f00d43bab7aea37d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::save_thread @ 0x172126a -> 0x171f7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::execute_save_cmd @ 0x172439f -> 0x171f7f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::CurrentSaveFile::perform_manual_backup
// 信息 ID: 9afa3bc46a196a49f98a42f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=87232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::CurrentSaveFile::replace_edits
// 信息 ID: 27ba2fdab34c19528825a969
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=72148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::UnsavedFileList::add
// 信息 ID: 0784d8940460b656cec5095d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::execute_save_cmd @ 0x17244d8 -> 0x171ff80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::UnsavedFileList::contains_file
// 信息 ID: 0c8828297e10ee2fcb24200a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::load_completion_checker::is_session_ok_to_load @ 0xa9fafa -> 0x17201a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::save_thread @ 0x1721085 -> 0x17201a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::capture_snapshot
// 信息 ID: 92c890f33c97e9e4d44b6781
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=96316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::execute_open_file_cmd
// 信息 ID: 6b734974f7788167e8ed34ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=102452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::execute_open_file_cmd::closure$0
// 信息 ID: 8b7e7f4b137424d13121effe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_thread::execute_open_file_cmd", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=105032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::execute_open_file_cmd::closure$2
// 信息 ID: 9593ad92fcdba5435daedfc9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::save::save_thread::execute_open_file_cmd", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2 <- std::sys::backtrace::__rust_begin_short_backtrace<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2 <- std::thread::Builder::spawn<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2 <- core::ptr::drop_in_place<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2,tuple$<> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2,tuple$<> >; module=506, inline_record=9824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::execute_save_cmd
// 信息 ID: bf1a6278a2b5fa810a1f86b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::save_thread @ 0x17207bc -> 0x1721aa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::full_write_history_file
// 信息 ID: 2ede3189780ff14244a1915f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1686, inline_record=113284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$0::backup_save::closure$1
// 信息 ID: 7519246a2651496c92bc4d8f
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
// 信息 ID: a8d8531f11d4566798f7271d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::save::save_thread::impl$0::backup_save", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2 <- core::slice::sort::stable::driftsort_main<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2>,alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2 <- core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2 <- core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2 <- core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; module=1592, inline_record=128424; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; module=1592, inline_record=128588; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; module=1592, inline_record=174516; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; module=1592, inline_record=174692; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; module=1592, inline_record=174808; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; module=1592, inline_record=174932; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; module=1592, inline_record=175104; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,country_core::systems::save::save_thread::impl$0::backup_save::closure_env$2> >; module=1592, inline_record=175548; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$0::perform_manual_backup::closure$0
// 信息 ID: cb91a8aff0016c1307481178
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_thread::impl$0::perform_manual_backup", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_thread::impl$0::perform_manual_backup::closure_env$0 <- anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,country_core::systems::save::save_thread::impl$0::perform_manual_backup::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,country_core::systems::save::save_thread::impl$0::perform_manual_backup::closure_env$0>; module=1629, inline_record=108016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$10::clone
// 信息 ID: 191b4cd88fd47d4082e8b150
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1686, inline_record=36220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$11::default
// 信息 ID: 8150ab65f083f0818162b683
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1686, inline_record=34228; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1686, inline_record=34432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$13::eq
// 信息 ID: f554255dc3b57b8a21a96cbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1686, inline_record=35176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$1::contains_file::closure$0
// 信息 ID: 012b5d158c7d39d7b5233201
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_thread::impl$1::contains_file", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::UnsavedFileList::contains_file; module=1686, inline_record=31488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$2::from
// 信息 ID: 39c98adcd7d2993eebc9ec13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::save_thread::closure$0 @ 0x130fa00 -> 0x1720490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::save_thread @ 0x172087b -> 0x1720490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$5::clone
// 信息 ID: 412a0ca5114fd0195b29cbcb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$11::clone<tuple$<std::path::PathBuf,enum2$<country_core::systems::save::save_thread::AutoSaveError> >,alloc::alloc::Global>; module=1585, inline_record=127880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::impl$8::eq
// 信息 ID: 77287b94d50c668f986ed1f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1686, inline_record=35440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::nuke_unreferenced_shapshots
// 信息 ID: 830842cacd5c9694e23fc2c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::snapshot_gc_thread; module=1686, inline_record=136720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::save_thread
// 信息 ID: 0573ec1b89fa9f60812a70ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0,tuple$<> > @ 0x91af1f -> 0x1720680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::save_thread::closure$0
// 信息 ID: 79fbf6836dfc0836ea1677c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_thread::save_thread", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_thread::save_thread::closure_env$0 <- alloc::vec::Vec::retain_mut<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global,country_core::systems::save::save_thread::save_thread::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] alloc::vec::Vec::retain_mut<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global,country_core::systems::save::save_thread::save_thread::closure_env$0> @ 0x12f3cbd -> 0x130f900 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::Vec::retain_mut<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global,country_core::systems::save::save_thread::save_thread::closure_env$0> @ 0x12f3d8a -> 0x130f900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::save_thread::closure$1
// 信息 ID: 44b5a388a8c9a51cf998fcf3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::save::save_thread::save_thread", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_thread::save_thread::closure_env$1 <- hashbrown::map::HashMap::retain<std::path::PathBuf,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,hashbrown::set::impl$1::retain::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,country_core::systems::save::save_thread::save_thread::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] hashbrown::map::HashMap::retain<std::path::PathBuf,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,hashbrown::set::impl$1::retain::closure_env$0<std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,country_core::systems::save::save_thread::save_thread::closure_env$1> >; module=1613, inline_record=229572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::save_thread::closure$2
// 信息 ID: 5b7eed46d30a11c4adb71d73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::save::save_thread::save_thread", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1686, inline_record=41500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_thread::snapshot_gc_thread
// 信息 ID: a41fc3a1354a55916310a939
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<country_core::systems::save::save_thread::execute_open_file_cmd::closure_env$2,tuple$<> > @ 0x91b3c7 -> 0x172b280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
