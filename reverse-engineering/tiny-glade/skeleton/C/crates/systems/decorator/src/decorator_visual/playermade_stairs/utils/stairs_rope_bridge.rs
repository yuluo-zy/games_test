// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 90be1128261d488bd2fa230334ece914b2324a229b2d47420372c7d264269568
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stairs_rope_bridge.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 432, 1545, 1557, 1600
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bracket_noise; // namespace candidate
//   package identities: [{"name":"bracket-noise","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.2"}]
// use core; // namespace candidate
//   package identities: []
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::impl$0::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":432,"record_offset":272,"rva":8683424,"va":5377392544}
// 编译实例 RVAs: 0x847fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bracket_noise::fastnoise::FastNoise::seeded
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bracket-noise","version":"0.8.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13028246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392544,"raw_hex":"e896cbc600","size":5,"soft_fail":false,"target_va":5390420816,"va":5377392565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bracket_noise::fastnoise::FastNoise::set_frequency
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bracket-noise","version":"0.8.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bracket-noise-0.8.2/src/fastnoise.rs
//     evidence={"module":432,"record_offset":488,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26729,"source_line":1809,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] bracket_noise::fastnoise::FastNoise::set_noise_type
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bracket-noise","version":"0.8.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bracket-noise-0.8.2/src/fastnoise.rs
//     evidence={"module":432,"record_offset":460,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26728,"source_line":1821,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::RopeBridgeNoise::sample
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6870
// 首条位置: {"module":432,"inlinee":26736}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":4192,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":4312,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":4340,"parent_record_offset":4312,"ancestor_record_offset":4160,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":4224,"parent_record_offset":4192,"ancestor_record_offset":4160,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::RopeBridgeNoise::phase
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x686F
// 首条位置: {"module":432,"inlinee":26735}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::RopeBridgeNoise::sample
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":26736,"source_line":48,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":4340,"parent_record_offset":4312,"ancestor_record_offset":4136,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":4224,"parent_record_offset":4192,"ancestor_record_offset":4136,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":4192,"parent_record_offset":4160,"ancestor_record_offset":4136,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":4312,"parent_record_offset":4160,"ancestor_record_offset":4136,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_noise_scale
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x686D
// 首条位置: {"module":432,"inlinee":26733}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":2032,"parent_record_offset":1984,"ancestor_record_offset":1984,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":432,"record_offset":2136,"parent_record_offset":1984,"ancestor_record_offset":1984,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":2296,"parent_record_offset":1984,"ancestor_record_offset":1984,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":2072,"parent_record_offset":1984,"ancestor_record_offset":1984,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":2100,"parent_record_offset":2072,"ancestor_record_offset":1984,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":2260,"parent_record_offset":2136,"ancestor_record_offset":1984,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":2196,"parent_record_offset":2136,"ancestor_record_offset":1984,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":2164,"parent_record_offset":2136,"ancestor_record_offset":1984,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge
// 记录形式: compiled_procedure; instances=1
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":432,"record_offset":11340,"rva":8688112,"va":5377397232}
// 编译实例 RVAs: 0x8491f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-1719573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e8ebc2e5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377398256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=44
//     evidence={"assembly":"\tcallq\t-1726029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e8b3a9e5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377404728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=8
//     target_aliases=1699 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8676049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e82f9d7bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377398236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fac70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5570387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e8ad00abff","size":5,"soft_fail":false,"target_va":5371833456,"va":5377403838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fda50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5562655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e8e11eabff","size":5,"soft_fail":false,"target_va":5371845200,"va":5377407850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t727345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e831190b00","size":5,"soft_fail":false,"target_va":5378126944,"va":5377399594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c663b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4313161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e849d04100","size":5,"soft_fail":false,"target_va":5381710768,"va":5377397602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4326962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e832064200","size":5,"soft_fail":false,"target_va":5381726576,"va":5377399609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5925590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e8d66a5a00","size":5,"soft_fail":false,"target_va":5383325024,"va":5377399429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a39b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12952119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e837a2c500","size":5,"soft_fail":false,"target_va":5390350768,"va":5377398644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414b6a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13028696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e858cdc600","size":5,"soft_fail":false,"target_va":5390428736,"va":5377400035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38943025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e831395202","size":5,"soft_fail":false,"target_va":5416350944,"va":5377407914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38946875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e83b485202","size":5,"soft_fail":false,"target_va":5416351392,"va":5377404512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38948632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e8184f5202","size":5,"soft_fail":false,"target_va":5416352064,"va":5377403427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39658747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e8fb245d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5377398304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39654513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e871145d02","size":5,"soft_fail":false,"target_va":5417057664,"va":5377403146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39658559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e83f245d02","size":5,"soft_fail":false,"target_va":5417057696,"va":5377399132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39658773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e815255d02","size":5,"soft_fail":false,"target_va":5417057792,"va":5377399014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39658884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e884255d02","size":5,"soft_fail":false,"target_va":5417058032,"va":5377399143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38962482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e832855202","size":5,"soft_fail":false,"target_va":5416366080,"va":5377403593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t39212482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e8c2555602","size":5,"soft_fail":false,"target_va":5416620794,"va":5377408307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t39213154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e862585602","size":5,"soft_fail":false,"target_va":5416621600,"va":5377408441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39212682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e88a565602","size":5,"soft_fail":false,"target_va":5416621188,"va":5377408501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=21
//     evidence={"assembly":"\tcallq\t39213355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e82b595602","size":5,"soft_fail":false,"target_va":5416621762,"va":5377408402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t39213966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e88e5b5602","size":5,"soft_fail":false,"target_va":5416622320,"va":5377408349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5984043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e82b4f5b00","size":5,"soft_fail":false,"target_va":5383383152,"va":5377399104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::swept_frame_samples
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t394675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e8b3050600","size":5,"soft_fail":false,"target_va":5377798752,"va":5377404072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e8b7d9ffff","size":5,"soft_fail":false,"target_va":5377392704,"va":5377402500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::pull_rope_tips_to_posts
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377397232,"raw_hex":"e84ee3ffff","size":5,"soft_fail":false,"target_va":5377395200,"va":5377402541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":432,"record_offset":11704,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":17740,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":12712,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":35552,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":432,"record_offset":35608,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":432,"record_offset":20432,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":432,"record_offset":12344,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":11608,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":14188,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":24348,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":432,"record_offset":13504,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":9810,"source_line":581,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":13448,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::stair_side_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs
//     evidence={"module":432,"record_offset":12140,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":26738,"source_line":340,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":16376,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":26752,"source_line":163,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::densify_rope_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":20732,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":26760,"source_line":410,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::parabolic_rope_panels
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":27532,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":26762,"source_line":325,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::queue_swept_rope
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":23032,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":26761,"source_line":504,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_post_top_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":12820,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":26739,"source_line":258,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_post_with_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":432,"record_offset":13720,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":26744,"source_line":774,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":11340,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":11964,"parent_record_offset":11940,"ancestor_record_offset":11340,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":22632,"parent_record_offset":22608,"ancestor_record_offset":11340,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":22656,"parent_record_offset":22632,"ancestor_record_offset":11340,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":11340,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":22680,"parent_record_offset":22656,"ancestor_record_offset":11340,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":11916,"parent_record_offset":11888,"ancestor_record_offset":11340,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":22608,"parent_record_offset":22516,"ancestor_record_offset":11340,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":21376,"parent_record_offset":21352,"ancestor_record_offset":11340,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":11828,"parent_record_offset":11800,"ancestor_record_offset":11340,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":22544,"parent_record_offset":22516,"ancestor_record_offset":11340,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":22516,"parent_record_offset":22492,"ancestor_record_offset":11340,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":21424,"parent_record_offset":21400,"ancestor_record_offset":11340,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":21400,"parent_record_offset":21376,"ancestor_record_offset":11340,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":11888,"parent_record_offset":11856,"ancestor_record_offset":11340,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":11856,"parent_record_offset":11828,"ancestor_record_offset":11340,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":22492,"parent_record_offset":22468,"ancestor_record_offset":11340,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":28724,"parent_record_offset":28692,"ancestor_record_offset":11340,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":432,"record_offset":11732,"parent_record_offset":11704,"ancestor_record_offset":11340,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":432,"record_offset":11760,"parent_record_offset":11732,"ancestor_record_offset":11340,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":21352,"parent_record_offset":21312,"ancestor_record_offset":11340,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":14764,"parent_record_offset":14732,"ancestor_record_offset":11340,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":18060,"parent_record_offset":17776,"ancestor_record_offset":11340,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":20848,"parent_record_offset":20732,"ancestor_record_offset":11340,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":21284,"parent_record_offset":20732,"ancestor_record_offset":11340,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":21312,"parent_record_offset":21284,"ancestor_record_offset":11340,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":12100,"parent_record_offset":11760,"ancestor_record_offset":11340,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":14396,"parent_record_offset":14332,"ancestor_record_offset":11340,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":11800,"parent_record_offset":11760,"ancestor_record_offset":11340,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":17028,"parent_record_offset":16656,"ancestor_record_offset":11340,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":12280,"parent_record_offset":12256,"ancestor_record_offset":11340,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":32076,"parent_record_offset":27532,"ancestor_record_offset":11340,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":32192,"parent_record_offset":27532,"ancestor_record_offset":11340,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":15232,"parent_record_offset":15196,"ancestor_record_offset":11340,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":34840,"parent_record_offset":34816,"ancestor_record_offset":11340,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":432,"record_offset":37760,"parent_record_offset":37384,"ancestor_record_offset":11340,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":432,"record_offset":37384,"parent_record_offset":37360,"ancestor_record_offset":11340,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":432,"record_offset":37980,"parent_record_offset":37956,"ancestor_record_offset":11340,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":432,"record_offset":37832,"parent_record_offset":37808,"ancestor_record_offset":11340,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":432,"record_offset":14732,"parent_record_offset":14332,"ancestor_record_offset":11340,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":432,"record_offset":17776,"parent_record_offset":17740,"ancestor_record_offset":11340,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":432,"record_offset":14284,"parent_record_offset":14236,"ancestor_record_offset":11340,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":432,"record_offset":14332,"parent_record_offset":14284,"ancestor_record_offset":11340,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":432,"record_offset":28748,"parent_record_offset":28724,"ancestor_record_offset":11340,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::array::from_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":432,"record_offset":16416,"parent_record_offset":16376,"ancestor_record_offset":11340,"inlinee":26753,"source_line":109,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":432,"record_offset":12388,"parent_record_offset":12344,"ancestor_record_offset":11340,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":432,"record_offset":12424,"parent_record_offset":12388,"ancestor_record_offset":11340,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":432,"record_offset":12584,"parent_record_offset":12460,"ancestor_record_offset":11340,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":432,"record_offset":16456,"parent_record_offset":16416,"ancestor_record_offset":11340,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":432,"record_offset":16496,"parent_record_offset":16456,"ancestor_record_offset":11340,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":432,"record_offset":23144,"parent_record_offset":23032,"ancestor_record_offset":11340,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":432,"record_offset":11664,"parent_record_offset":11636,"ancestor_record_offset":11340,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":432,"record_offset":14152,"parent_record_offset":13804,"ancestor_record_offset":11340,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":13148,"parent_record_offset":12820,"ancestor_record_offset":11340,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":21256,"parent_record_offset":20732,"ancestor_record_offset":11340,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":21224,"parent_record_offset":21200,"ancestor_record_offset":11340,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":12940,"parent_record_offset":12916,"ancestor_record_offset":11340,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":13668,"parent_record_offset":13644,"ancestor_record_offset":11340,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":432,"record_offset":28800,"parent_record_offset":28772,"ancestor_record_offset":11340,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":432,"record_offset":14828,"parent_record_offset":14796,"ancestor_record_offset":11340,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":432,"record_offset":14952,"parent_record_offset":14860,"ancestor_record_offset":11340,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":432,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":11340,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":432,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":11340,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":18220,"parent_record_offset":18144,"ancestor_record_offset":11340,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":11636,"parent_record_offset":11608,"ancestor_record_offset":11340,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":18108,"parent_record_offset":18060,"ancestor_record_offset":11340,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":432,"record_offset":12304,"parent_record_offset":12280,"ancestor_record_offset":11340,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":432,"record_offset":37808,"parent_record_offset":37784,"ancestor_record_offset":11340,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":432,"record_offset":37784,"parent_record_offset":37760,"ancestor_record_offset":11340,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":12256,"parent_record_offset":12140,"ancestor_record_offset":11340,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":14796,"parent_record_offset":14764,"ancestor_record_offset":11340,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":15196,"parent_record_offset":14952,"ancestor_record_offset":11340,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":12172,"parent_record_offset":12140,"ancestor_record_offset":11340,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":432,"record_offset":32104,"parent_record_offset":32076,"ancestor_record_offset":11340,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":432,"record_offset":32128,"parent_record_offset":32104,"ancestor_record_offset":11340,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":432,"record_offset":12608,"parent_record_offset":12584,"ancestor_record_offset":11340,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":28772,"parent_record_offset":28748,"ancestor_record_offset":11340,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":23260,"parent_record_offset":23212,"ancestor_record_offset":11340,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":28828,"parent_record_offset":28772,"ancestor_record_offset":11340,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":12552,"parent_record_offset":12516,"ancestor_record_offset":11340,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":22576,"parent_record_offset":22544,"ancestor_record_offset":11340,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":14000,"parent_record_offset":13864,"ancestor_record_offset":11340,"inlinee":26749,"source_line":2532,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":13964,"parent_record_offset":13932,"ancestor_record_offset":11340,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":13472,"parent_record_offset":13448,"ancestor_record_offset":11340,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":13208,"parent_record_offset":13180,"ancestor_record_offset":11340,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":13772,"parent_record_offset":13748,"ancestor_record_offset":11340,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":432,"record_offset":12516,"parent_record_offset":12460,"ancestor_record_offset":11340,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":432,"record_offset":12460,"parent_record_offset":12424,"ancestor_record_offset":11340,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":432,"record_offset":16536,"parent_record_offset":16496,"ancestor_record_offset":11340,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":432,"record_offset":16616,"parent_record_offset":16576,"ancestor_record_offset":11340,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":432,"record_offset":21936,"parent_record_offset":20732,"ancestor_record_offset":11340,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":432,"record_offset":12044,"parent_record_offset":11760,"ancestor_record_offset":11340,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":432,"record_offset":37904,"parent_record_offset":37880,"ancestor_record_offset":11340,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":432,"record_offset":12632,"parent_record_offset":12608,"ancestor_record_offset":11340,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":12068,"parent_record_offset":12044,"ancestor_record_offset":11340,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=69
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":22444,"parent_record_offset":20732,"ancestor_record_offset":11340,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":432,"record_offset":18316,"parent_record_offset":18060,"ancestor_record_offset":11340,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":432,"record_offset":37880,"parent_record_offset":37856,"ancestor_record_offset":11340,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":432,"record_offset":14920,"parent_record_offset":14860,"ancestor_record_offset":11340,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":432,"record_offset":37856,"parent_record_offset":37832,"ancestor_record_offset":11340,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":432,"record_offset":35696,"parent_record_offset":35608,"ancestor_record_offset":11340,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":432,"record_offset":23372,"parent_record_offset":23324,"ancestor_record_offset":11340,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":12656,"parent_record_offset":12632,"ancestor_record_offset":11340,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":15268,"parent_record_offset":15232,"ancestor_record_offset":11340,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":432,"record_offset":20596,"parent_record_offset":20572,"ancestor_record_offset":11340,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":432,"record_offset":21908,"parent_record_offset":20732,"ancestor_record_offset":11340,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":23288,"parent_record_offset":23212,"ancestor_record_offset":11340,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":12744,"parent_record_offset":12712,"ancestor_record_offset":11340,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":23212,"parent_record_offset":23176,"ancestor_record_offset":11340,"inlinee":11671,"source_line":696,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":31356,"parent_record_offset":31328,"ancestor_record_offset":11340,"inlinee":26764,"source_line":793,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":20628,"parent_record_offset":20596,"ancestor_record_offset":11340,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":12776,"parent_record_offset":12744,"ancestor_record_offset":11340,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":31752,"parent_record_offset":31724,"ancestor_record_offset":11340,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":432,"record_offset":14860,"parent_record_offset":14828,"ancestor_record_offset":11340,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":432,"record_offset":23324,"parent_record_offset":23032,"ancestor_record_offset":11340,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":432,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":11340,"inlinee":26747,"source_line":143,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":432,"record_offset":13908,"parent_record_offset":13864,"ancestor_record_offset":11340,"inlinee":26748,"source_line":122,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":432,"record_offset":13932,"parent_record_offset":13908,"ancestor_record_offset":11340,"inlinee":6595,"source_line":128,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":432,"record_offset":13840,"parent_record_offset":13804,"ancestor_record_offset":11340,"inlinee":26746,"source_line":237,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":432,"record_offset":12892,"parent_record_offset":12864,"ancestor_record_offset":11340,"inlinee":26741,"source_line":222,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_ivec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":432,"record_offset":13064,"parent_record_offset":12820,"ancestor_record_offset":11340,"inlinee":26742,"source_line":1081,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":432,"record_offset":13612,"parent_record_offset":13580,"ancestor_record_offset":11340,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":432,"record_offset":12864,"parent_record_offset":12820,"ancestor_record_offset":11340,"inlinee":26740,"source_line":687,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":432,"record_offset":13580,"parent_record_offset":13556,"ancestor_record_offset":11340,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":432,"record_offset":13556,"parent_record_offset":13528,"ancestor_record_offset":11340,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":432,"record_offset":13528,"parent_record_offset":13504,"ancestor_record_offset":11340,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":20988,"parent_record_offset":20732,"ancestor_record_offset":11340,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":21100,"parent_record_offset":21012,"ancestor_record_offset":11340,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":13180,"parent_record_offset":12820,"ancestor_record_offset":11340,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":15020,"parent_record_offset":14984,"ancestor_record_offset":11340,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":13748,"parent_record_offset":13720,"ancestor_record_offset":11340,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":432,"record_offset":21824,"parent_record_offset":20732,"ancestor_record_offset":11340,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":21200,"parent_record_offset":20732,"ancestor_record_offset":11340,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":13092,"parent_record_offset":12820,"ancestor_record_offset":11340,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":12916,"parent_record_offset":12892,"ancestor_record_offset":11340,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":13120,"parent_record_offset":12820,"ancestor_record_offset":11340,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":13644,"parent_record_offset":13612,"ancestor_record_offset":11340,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":14984,"parent_record_offset":14952,"ancestor_record_offset":11340,"inlinee":26751,"source_line":154,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":16004,"parent_record_offset":15972,"ancestor_record_offset":11340,"inlinee":26756,"source_line":158,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":16576,"parent_record_offset":16536,"ancestor_record_offset":11340,"inlinee":26755,"source_line":164,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$2::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":16656,"parent_record_offset":16616,"ancestor_record_offset":11340,"inlinee":26758,"source_line":170,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_post_attachment_target
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":16692,"parent_record_offset":16656,"ancestor_record_offset":11340,"inlinee":26759,"source_line":268,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::push_wooden_railing_piece
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs
//     evidence={"module":432,"record_offset":13804,"parent_record_offset":13720,"ancestor_record_offset":11340,"inlinee":26745,"source_line":825,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x687F
// 首条位置: {"module":432,"inlinee":26751}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":15020,"parent_record_offset":14984,"ancestor_record_offset":14984,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":15080,"parent_record_offset":15020,"ancestor_record_offset":14984,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6884
// 首条位置: {"module":432,"inlinee":26756}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":16040,"parent_record_offset":16004,"ancestor_record_offset":16004,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":16100,"parent_record_offset":16040,"ancestor_record_offset":16004,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6880
// 首条位置: {"module":432,"inlinee":26752}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::from_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":432,"record_offset":16416,"parent_record_offset":16376,"ancestor_record_offset":16376,"inlinee":26753,"source_line":109,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":17028,"parent_record_offset":16656,"ancestor_record_offset":16376,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":432,"record_offset":16456,"parent_record_offset":16416,"ancestor_record_offset":16376,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":432,"record_offset":16496,"parent_record_offset":16456,"ancestor_record_offset":16376,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":16904,"parent_record_offset":16692,"ancestor_record_offset":16376,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":16752,"parent_record_offset":16728,"ancestor_record_offset":16376,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":16964,"parent_record_offset":16936,"ancestor_record_offset":16376,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":432,"record_offset":16536,"parent_record_offset":16496,"ancestor_record_offset":16376,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":432,"record_offset":16616,"parent_record_offset":16576,"ancestor_record_offset":16376,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":17052,"parent_record_offset":17028,"ancestor_record_offset":16376,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":17076,"parent_record_offset":17052,"ancestor_record_offset":16376,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":16936,"parent_record_offset":16692,"ancestor_record_offset":16376,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":16728,"parent_record_offset":16692,"ancestor_record_offset":16376,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$2::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":16576,"parent_record_offset":16536,"ancestor_record_offset":16376,"inlinee":26755,"source_line":164,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$2::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":16656,"parent_record_offset":16616,"ancestor_record_offset":16376,"inlinee":26758,"source_line":170,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_post_attachment_target
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":16692,"parent_record_offset":16656,"ancestor_record_offset":16376,"inlinee":26759,"source_line":268,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6883
// 首条位置: {"module":432,"inlinee":26755}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":432,"record_offset":16616,"parent_record_offset":16576,"ancestor_record_offset":16576,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":17028,"parent_record_offset":16656,"ancestor_record_offset":16576,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":16904,"parent_record_offset":16692,"ancestor_record_offset":16576,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":16752,"parent_record_offset":16728,"ancestor_record_offset":16576,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":16964,"parent_record_offset":16936,"ancestor_record_offset":16576,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":17052,"parent_record_offset":17028,"ancestor_record_offset":16576,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":17076,"parent_record_offset":17052,"ancestor_record_offset":16576,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":16936,"parent_record_offset":16692,"ancestor_record_offset":16576,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":16728,"parent_record_offset":16692,"ancestor_record_offset":16576,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$2::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":16656,"parent_record_offset":16616,"ancestor_record_offset":16576,"inlinee":26758,"source_line":170,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_post_attachment_target
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":16692,"parent_record_offset":16656,"ancestor_record_offset":16576,"inlinee":26759,"source_line":268,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$2::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6886
// 首条位置: {"module":432,"inlinee":26758}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":17028,"parent_record_offset":16656,"ancestor_record_offset":16656,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_post_attachment_target
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":16692,"parent_record_offset":16656,"ancestor_record_offset":16656,"inlinee":26759,"source_line":268,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":16904,"parent_record_offset":16692,"ancestor_record_offset":16656,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":16752,"parent_record_offset":16728,"ancestor_record_offset":16656,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":16964,"parent_record_offset":16936,"ancestor_record_offset":16656,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":17052,"parent_record_offset":17028,"ancestor_record_offset":16656,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":17076,"parent_record_offset":17052,"ancestor_record_offset":16656,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":16936,"parent_record_offset":16692,"ancestor_record_offset":16656,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":16728,"parent_record_offset":16692,"ancestor_record_offset":16656,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_post_top_offset
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6873
// 首条位置: {"module":432,"inlinee":26739}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":13148,"parent_record_offset":12820,"ancestor_record_offset":12820,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_ivec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":432,"record_offset":13064,"parent_record_offset":12820,"ancestor_record_offset":12820,"inlinee":26742,"source_line":1081,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":432,"record_offset":12864,"parent_record_offset":12820,"ancestor_record_offset":12820,"inlinee":26740,"source_line":687,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":13180,"parent_record_offset":12820,"ancestor_record_offset":12820,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":13092,"parent_record_offset":12820,"ancestor_record_offset":12820,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":13120,"parent_record_offset":12820,"ancestor_record_offset":12820,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":12940,"parent_record_offset":12916,"ancestor_record_offset":12820,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":13208,"parent_record_offset":13180,"ancestor_record_offset":12820,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":432,"record_offset":12892,"parent_record_offset":12864,"ancestor_record_offset":12820,"inlinee":26741,"source_line":222,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":12916,"parent_record_offset":12892,"ancestor_record_offset":12820,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_post_attachment_target
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6887
// 首条位置: {"module":432,"inlinee":26759}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":16904,"parent_record_offset":16692,"ancestor_record_offset":16692,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":16936,"parent_record_offset":16692,"ancestor_record_offset":16692,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":16728,"parent_record_offset":16692,"ancestor_record_offset":16692,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":16752,"parent_record_offset":16728,"ancestor_record_offset":16692,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":16964,"parent_record_offset":16936,"ancestor_record_offset":16692,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::pull_rope_tips_to_posts
// 记录形式: compiled_procedure; instances=1
// 源码行: 280
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":432,"record_offset":6812,"rva":8686080,"va":5377395200}
// 编译实例 RVAs: 0x848a00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1716626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377395200,"raw_hex":"e86ecee5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377395309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-1717752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377395200,"raw_hex":"e808cae5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377396451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8673109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377395200,"raw_hex":"e8aba87bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377395296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39661716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377395200,"raw_hex":"e894305d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5377395335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39224052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377395200,"raw_hex":"e8f4825602","size":5,"soft_fail":false,"target_va":5416620794,"va":5377396737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39224924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377395200,"raw_hex":"e85c865602","size":5,"soft_fail":false,"target_va":5416621600,"va":5377396671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39225036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377395200,"raw_hex":"e8cc865602","size":5,"soft_fail":false,"target_va":5416621762,"va":5377396721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-1717905","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377395200,"raw_hex":"e96fc9e5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377396604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":432,"record_offset":7060,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":7672,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":9252,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":8372,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":432,"record_offset":8236,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":15318,"source_line":54,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":432,"record_offset":8292,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":432,"record_offset":8952,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":9752,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":432,"record_offset":10432,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":10032,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":432,"record_offset":8844,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":9372,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":9648,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":8672,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":8564,"parent_record_offset":6812,"ancestor_record_offset":6812,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":7500,"parent_record_offset":7476,"ancestor_record_offset":6812,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":7524,"parent_record_offset":7500,"ancestor_record_offset":6812,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":10312,"parent_record_offset":10288,"ancestor_record_offset":6812,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":10336,"parent_record_offset":10312,"ancestor_record_offset":6812,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":7548,"parent_record_offset":7524,"ancestor_record_offset":6812,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":10360,"parent_record_offset":10336,"ancestor_record_offset":6812,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":7476,"parent_record_offset":7312,"ancestor_record_offset":6812,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":10288,"parent_record_offset":10200,"ancestor_record_offset":6812,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":7228,"parent_record_offset":7188,"ancestor_record_offset":6812,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":10228,"parent_record_offset":10200,"ancestor_record_offset":6812,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":10200,"parent_record_offset":10176,"ancestor_record_offset":6812,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":7312,"parent_record_offset":7268,"ancestor_record_offset":6812,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":7268,"parent_record_offset":7228,"ancestor_record_offset":6812,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":10176,"parent_record_offset":10152,"ancestor_record_offset":6812,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":7364,"parent_record_offset":7312,"ancestor_record_offset":6812,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":432,"record_offset":7100,"parent_record_offset":7060,"ancestor_record_offset":6812,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":432,"record_offset":7140,"parent_record_offset":7100,"ancestor_record_offset":6812,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":7984,"parent_record_offset":7704,"ancestor_record_offset":6812,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":7188,"parent_record_offset":7140,"ancestor_record_offset":6812,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":432,"record_offset":10080,"parent_record_offset":10056,"ancestor_record_offset":6812,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":432,"record_offset":10128,"parent_record_offset":10104,"ancestor_record_offset":6812,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":432,"record_offset":9096,"parent_record_offset":8988,"ancestor_record_offset":6812,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":432,"record_offset":7704,"parent_record_offset":7672,"ancestor_record_offset":6812,"inlinee":6996,"source_line":13,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":432,"record_offset":7388,"parent_record_offset":7364,"ancestor_record_offset":6812,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::new_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":432,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":6812,"inlinee":15319,"source_line":94,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":432,"record_offset":8096,"parent_record_offset":8064,"ancestor_record_offset":6812,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":9600,"parent_record_offset":9572,"ancestor_record_offset":6812,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":432,"record_offset":8988,"parent_record_offset":8952,"ancestor_record_offset":6812,"inlinee":6658,"source_line":164,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":8064,"parent_record_offset":8032,"ancestor_record_offset":6812,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":8032,"parent_record_offset":7984,"ancestor_record_offset":6812,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":7412,"parent_record_offset":7388,"ancestor_record_offset":6812,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":7436,"parent_record_offset":7412,"ancestor_record_offset":6812,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":10256,"parent_record_offset":10228,"ancestor_record_offset":6812,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":9684,"parent_record_offset":9648,"ancestor_record_offset":6812,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":8744,"parent_record_offset":8672,"ancestor_record_offset":6812,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":8604,"parent_record_offset":8564,"ancestor_record_offset":6812,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":432,"record_offset":8328,"parent_record_offset":8292,"ancestor_record_offset":6812,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":432,"record_offset":7604,"parent_record_offset":7140,"ancestor_record_offset":6812,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":6812,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":6812,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":432,"record_offset":8164,"parent_record_offset":7984,"ancestor_record_offset":6812,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":432,"record_offset":9212,"parent_record_offset":9096,"ancestor_record_offset":6812,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":432,"record_offset":9156,"parent_record_offset":9096,"ancestor_record_offset":6812,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":432,"record_offset":9064,"parent_record_offset":9028,"ancestor_record_offset":6812,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":9180,"parent_record_offset":9156,"ancestor_record_offset":6812,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":8132,"parent_record_offset":7984,"ancestor_record_offset":6812,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":9288,"parent_record_offset":9252,"ancestor_record_offset":6812,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":8432,"parent_record_offset":8372,"ancestor_record_offset":6812,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":9324,"parent_record_offset":9288,"ancestor_record_offset":6812,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":8492,"parent_record_offset":8432,"ancestor_record_offset":6812,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":432,"record_offset":8876,"parent_record_offset":8844,"ancestor_record_offset":6812,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":432,"record_offset":9028,"parent_record_offset":8988,"ancestor_record_offset":6812,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":9524,"parent_record_offset":9408,"ancestor_record_offset":6812,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":9408,"parent_record_offset":9372,"ancestor_record_offset":6812,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":9572,"parent_record_offset":9524,"ancestor_record_offset":6812,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::parabolic_rope_panels
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x688A
// 首条位置: {"module":432,"inlinee":26762}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":27836,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":29664,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":28576,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":31300,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":32076,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":32192,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":28464,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":29880,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":32772,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":28040,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":27952,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":27864,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":432,"record_offset":30520,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":28492,"parent_record_offset":27532,"ancestor_record_offset":27532,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":28892,"parent_record_offset":28868,"ancestor_record_offset":27532,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":28916,"parent_record_offset":28892,"ancestor_record_offset":27532,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":32660,"parent_record_offset":32636,"ancestor_record_offset":27532,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":32684,"parent_record_offset":32660,"ancestor_record_offset":27532,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":28940,"parent_record_offset":28916,"ancestor_record_offset":27532,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":32708,"parent_record_offset":32684,"ancestor_record_offset":27532,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":28868,"parent_record_offset":28692,"ancestor_record_offset":27532,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":32636,"parent_record_offset":32548,"ancestor_record_offset":27532,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":29756,"parent_record_offset":29732,"ancestor_record_offset":27532,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":28632,"parent_record_offset":28604,"ancestor_record_offset":27532,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":32576,"parent_record_offset":32548,"ancestor_record_offset":27532,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":32548,"parent_record_offset":32524,"ancestor_record_offset":27532,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":29804,"parent_record_offset":29780,"ancestor_record_offset":27532,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":29780,"parent_record_offset":29756,"ancestor_record_offset":27532,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":28692,"parent_record_offset":28660,"ancestor_record_offset":27532,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":28660,"parent_record_offset":28632,"ancestor_record_offset":27532,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":32524,"parent_record_offset":32500,"ancestor_record_offset":27532,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":28724,"parent_record_offset":28692,"ancestor_record_offset":27532,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":29732,"parent_record_offset":29692,"ancestor_record_offset":27532,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":29692,"parent_record_offset":29664,"ancestor_record_offset":27532,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":28604,"parent_record_offset":28576,"ancestor_record_offset":27532,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":34840,"parent_record_offset":34816,"ancestor_record_offset":27532,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":432,"record_offset":28748,"parent_record_offset":28724,"ancestor_record_offset":27532,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":432,"record_offset":29928,"parent_record_offset":29904,"ancestor_record_offset":27532,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":28516,"parent_record_offset":28492,"ancestor_record_offset":27532,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":28208,"parent_record_offset":28184,"ancestor_record_offset":27532,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":432,"record_offset":28800,"parent_record_offset":28772,"ancestor_record_offset":27532,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":29904,"parent_record_offset":29880,"ancestor_record_offset":27532,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":432,"record_offset":32216,"parent_record_offset":32192,"ancestor_record_offset":27532,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":432,"record_offset":32104,"parent_record_offset":32076,"ancestor_record_offset":27532,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":432,"record_offset":32128,"parent_record_offset":32104,"ancestor_record_offset":27532,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":28772,"parent_record_offset":28748,"ancestor_record_offset":27532,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":28828,"parent_record_offset":28772,"ancestor_record_offset":27532,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":32604,"parent_record_offset":32576,"ancestor_record_offset":27532,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":27892,"parent_record_offset":27864,"ancestor_record_offset":27532,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":27980,"parent_record_offset":27952,"ancestor_record_offset":27532,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":28096,"parent_record_offset":28064,"ancestor_record_offset":27532,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":32476,"parent_record_offset":32280,"ancestor_record_offset":27532,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":432,"record_offset":32160,"parent_record_offset":32076,"ancestor_record_offset":27532,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":29844,"parent_record_offset":29692,"ancestor_record_offset":27532,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":31896,"parent_record_offset":31860,"ancestor_record_offset":27532,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":31328,"parent_record_offset":31300,"ancestor_record_offset":27532,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":31384,"parent_record_offset":31356,"ancestor_record_offset":27532,"inlinee":11671,"source_line":696,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":31356,"parent_record_offset":31328,"ancestor_record_offset":27532,"inlinee":26764,"source_line":793,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":31752,"parent_record_offset":31724,"ancestor_record_offset":27532,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":28152,"parent_record_offset":28064,"ancestor_record_offset":27532,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":28064,"parent_record_offset":28040,"ancestor_record_offset":27532,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":28184,"parent_record_offset":28152,"ancestor_record_offset":27532,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::densify_rope_curve
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 410
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6888
// 首条位置: {"module":432,"inlinee":26760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":20848,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":21284,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":21256,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":20904,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":432,"record_offset":21936,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":22444,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":432,"record_offset":21908,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":20988,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":432,"record_offset":21824,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":21200,"parent_record_offset":20732,"ancestor_record_offset":20732,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":22632,"parent_record_offset":22608,"ancestor_record_offset":20732,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":22656,"parent_record_offset":22632,"ancestor_record_offset":20732,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":22680,"parent_record_offset":22656,"ancestor_record_offset":20732,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":22608,"parent_record_offset":22516,"ancestor_record_offset":20732,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":21376,"parent_record_offset":21352,"ancestor_record_offset":20732,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":22544,"parent_record_offset":22516,"ancestor_record_offset":20732,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":22516,"parent_record_offset":22492,"ancestor_record_offset":20732,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":21424,"parent_record_offset":21400,"ancestor_record_offset":20732,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":21400,"parent_record_offset":21376,"ancestor_record_offset":20732,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":22492,"parent_record_offset":22468,"ancestor_record_offset":20732,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":21352,"parent_record_offset":21312,"ancestor_record_offset":20732,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":21312,"parent_record_offset":21284,"ancestor_record_offset":20732,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":432,"record_offset":20952,"parent_record_offset":20928,"ancestor_record_offset":20732,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":21224,"parent_record_offset":21200,"ancestor_record_offset":20732,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":21156,"parent_record_offset":21132,"ancestor_record_offset":20732,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":20928,"parent_record_offset":20904,"ancestor_record_offset":20732,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":22576,"parent_record_offset":22544,"ancestor_record_offset":20732,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":21848,"parent_record_offset":21824,"ancestor_record_offset":20732,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":21044,"parent_record_offset":21012,"ancestor_record_offset":20732,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":22468,"parent_record_offset":22444,"ancestor_record_offset":20732,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":21464,"parent_record_offset":21312,"ancestor_record_offset":20732,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":21100,"parent_record_offset":21012,"ancestor_record_offset":20732,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":21012,"parent_record_offset":20988,"ancestor_record_offset":20732,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":21132,"parent_record_offset":21100,"ancestor_record_offset":20732,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 434
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":432,"record_offset":520,"rva":8683584,"va":5377392704}
// 编译实例 RVAs: 0x848040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-1714173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392704,"raw_hex":"e803d8e5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377392856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-1715664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392704,"raw_hex":"e830d2e5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377394363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8670656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392704,"raw_hex":"e840b27bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377392843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a4140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12958365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392704,"raw_hex":"e89dbac500","size":5,"soft_fail":false,"target_va":5390352704,"va":5377394334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bdd0
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39664551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392704,"raw_hex":"e8a73b5d02","size":5,"soft_fail":false,"target_va":5417057744,"va":5377393188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39663722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392704,"raw_hex":"e86a385d02","size":5,"soft_fail":false,"target_va":5417057840,"va":5377394113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stairs_rope_bridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39663814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392704,"raw_hex":"e8c6385d02","size":5,"soft_fail":false,"target_va":5417057984,"va":5377394165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38972672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392704,"raw_hex":"e800ad5202","size":5,"soft_fail":false,"target_va":5416366080,"va":5377393403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39227863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392704,"raw_hex":"e8d7915602","size":5,"soft_fail":false,"target_va":5416620794,"va":5377392926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bracket_noise::fastnoise::FastNoise::get_noise3d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bracket-noise","version":"0.8.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13002652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392704,"raw_hex":"e89c67c600","size":5,"soft_fail":false,"target_va":5390396624,"va":5377393967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39227084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392704,"raw_hex":"e8cc8e5602","size":5,"soft_fail":false,"target_va":5416621600,"va":5377394511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t39227210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377392704,"raw_hex":"e84a8f5602","size":5,"soft_fail":false,"target_va":5416621762,"va":5377394547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":2504,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":756,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":3468,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::recip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":3660,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":9829,"source_line":907,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":3144,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":1312,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":432,"record_offset":4464,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":5056,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":432,"record_offset":4436,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":3228,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":4020,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::rem_euclid
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":4856,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":11231,"source_line":289,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":4916,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::RopeBridgeNoise::phase
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":4136,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":26735,"source_line":54,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":520,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":520,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":5244,"parent_record_offset":5220,"ancestor_record_offset":520,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":5268,"parent_record_offset":5244,"ancestor_record_offset":520,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":520,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":520,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":912,"parent_record_offset":880,"ancestor_record_offset":520,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":5220,"parent_record_offset":5128,"ancestor_record_offset":520,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":520,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":820,"parent_record_offset":788,"ancestor_record_offset":520,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":5156,"parent_record_offset":5128,"ancestor_record_offset":520,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":520,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":520,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":520,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":880,"parent_record_offset":848,"ancestor_record_offset":520,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":848,"parent_record_offset":820,"ancestor_record_offset":520,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":520,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":2572,"parent_record_offset":2532,"ancestor_record_offset":520,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":1800,"parent_record_offset":1772,"ancestor_record_offset":520,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":2532,"parent_record_offset":2504,"ancestor_record_offset":520,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":1468,"parent_record_offset":1420,"ancestor_record_offset":520,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":788,"parent_record_offset":756,"ancestor_record_offset":520,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":1348,"parent_record_offset":1312,"ancestor_record_offset":520,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":2364,"parent_record_offset":2336,"ancestor_record_offset":520,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":432,"record_offset":2428,"parent_record_offset":2364,"ancestor_record_offset":520,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":432,"record_offset":1772,"parent_record_offset":1420,"ancestor_record_offset":520,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":432,"record_offset":1384,"parent_record_offset":1348,"ancestor_record_offset":520,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":432,"record_offset":1420,"parent_record_offset":1384,"ancestor_record_offset":520,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":432,"record_offset":3192,"parent_record_offset":3168,"ancestor_record_offset":520,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":2032,"parent_record_offset":1984,"ancestor_record_offset":520,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":2100,"parent_record_offset":2072,"ancestor_record_offset":520,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::rem_euclid
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":520,"inlinee":11232,"source_line":1982,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":520,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":520,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":432,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":520,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":432,"record_offset":1928,"parent_record_offset":1884,"ancestor_record_offset":520,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":3168,"parent_record_offset":3144,"ancestor_record_offset":520,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":432,"record_offset":5020,"parent_record_offset":4996,"ancestor_record_offset":520,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":1828,"parent_record_offset":1800,"ancestor_record_offset":520,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":2336,"parent_record_offset":1928,"ancestor_record_offset":520,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":5188,"parent_record_offset":5156,"ancestor_record_offset":520,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":2260,"parent_record_offset":2136,"ancestor_record_offset":520,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":2196,"parent_record_offset":2136,"ancestor_record_offset":520,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":520,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":2164,"parent_record_offset":2136,"ancestor_record_offset":520,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":2396,"parent_record_offset":2364,"ancestor_record_offset":520,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":3496,"parent_record_offset":3468,"ancestor_record_offset":520,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":520,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":432,"record_offset":1884,"parent_record_offset":1856,"ancestor_record_offset":520,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":3368,"parent_record_offset":3252,"ancestor_record_offset":520,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":4192,"parent_record_offset":4160,"ancestor_record_offset":520,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":4312,"parent_record_offset":4160,"ancestor_record_offset":520,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":3252,"parent_record_offset":3228,"ancestor_record_offset":520,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":432,"record_offset":2136,"parent_record_offset":1984,"ancestor_record_offset":520,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":2296,"parent_record_offset":1984,"ancestor_record_offset":520,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":2072,"parent_record_offset":1984,"ancestor_record_offset":520,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":3400,"parent_record_offset":3368,"ancestor_record_offset":520,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::RopeBridgeNoise::sample
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":520,"inlinee":26736,"source_line":48,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":1956,"parent_record_offset":1928,"ancestor_record_offset":520,"inlinee":26731,"source_line":446,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_noise_scale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":1984,"parent_record_offset":1956,"ancestor_record_offset":520,"inlinee":26733,"source_line":59,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 446
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x686B
// 首条位置: {"module":432,"inlinee":26731}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_noise_scale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":432,"record_offset":1984,"parent_record_offset":1956,"ancestor_record_offset":1956,"inlinee":26733,"source_line":59,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":2032,"parent_record_offset":1984,"ancestor_record_offset":1956,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":2100,"parent_record_offset":2072,"ancestor_record_offset":1956,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":2260,"parent_record_offset":2136,"ancestor_record_offset":1956,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":2196,"parent_record_offset":2136,"ancestor_record_offset":1956,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":2164,"parent_record_offset":2136,"ancestor_record_offset":1956,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":432,"record_offset":2136,"parent_record_offset":1984,"ancestor_record_offset":1956,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":2296,"parent_record_offset":1984,"ancestor_record_offset":1956,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":432,"record_offset":2072,"parent_record_offset":1984,"ancestor_record_offset":1956,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 476
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13B50
// 首条位置: {"module":1600,"inlinee":80720}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":45876,"parent_record_offset":45064,"ancestor_record_offset":45064,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1600,"record_offset":45228,"parent_record_offset":45064,"ancestor_record_offset":45064,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1600,"record_offset":45340,"parent_record_offset":45064,"ancestor_record_offset":45064,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1600,"record_offset":45492,"parent_record_offset":45064,"ancestor_record_offset":45064,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1600,"record_offset":45612,"parent_record_offset":45064,"ancestor_record_offset":45064,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1600,"record_offset":45200,"parent_record_offset":45064,"ancestor_record_offset":45064,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1600,"record_offset":45972,"parent_record_offset":45064,"ancestor_record_offset":45064,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::RopeBridgeNoise::sample
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs
//     evidence={"module":1600,"record_offset":46096,"parent_record_offset":45064,"ancestor_record_offset":45064,"inlinee":26736,"source_line":48,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1600,"record_offset":45364,"parent_record_offset":45340,"ancestor_record_offset":45064,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1600,"record_offset":45252,"parent_record_offset":45228,"ancestor_record_offset":45064,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1600,"record_offset":45904,"parent_record_offset":45876,"ancestor_record_offset":45064,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1600,"record_offset":45932,"parent_record_offset":45904,"ancestor_record_offset":45064,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1600,"record_offset":46136,"parent_record_offset":46096,"ancestor_record_offset":45064,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1600,"record_offset":46256,"parent_record_offset":46096,"ancestor_record_offset":45064,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::queue_swept_rope
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6889
// 首条位置: {"module":432,"inlinee":26761}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":23504,"parent_record_offset":23032,"ancestor_record_offset":23032,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":432,"record_offset":23144,"parent_record_offset":23032,"ancestor_record_offset":23032,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":432,"record_offset":23468,"parent_record_offset":23032,"ancestor_record_offset":23032,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":24060,"parent_record_offset":23032,"ancestor_record_offset":23032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":432,"record_offset":23440,"parent_record_offset":23032,"ancestor_record_offset":23032,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":23176,"parent_record_offset":23032,"ancestor_record_offset":23032,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":432,"record_offset":23324,"parent_record_offset":23032,"ancestor_record_offset":23032,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":23944,"parent_record_offset":23920,"ancestor_record_offset":23032,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":23968,"parent_record_offset":23944,"ancestor_record_offset":23032,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":23992,"parent_record_offset":23968,"ancestor_record_offset":23032,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":23920,"parent_record_offset":23832,"ancestor_record_offset":23032,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":23620,"parent_record_offset":23584,"ancestor_record_offset":23032,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":23860,"parent_record_offset":23832,"ancestor_record_offset":23032,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":23832,"parent_record_offset":23808,"ancestor_record_offset":23032,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":23668,"parent_record_offset":23644,"ancestor_record_offset":23032,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":23644,"parent_record_offset":23620,"ancestor_record_offset":23032,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":432,"record_offset":23808,"parent_record_offset":23784,"ancestor_record_offset":23032,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":23536,"parent_record_offset":23504,"ancestor_record_offset":23032,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":23260,"parent_record_offset":23212,"ancestor_record_offset":23032,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":23888,"parent_record_offset":23860,"ancestor_record_offset":23032,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":23736,"parent_record_offset":23536,"ancestor_record_offset":23032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":432,"record_offset":23584,"parent_record_offset":23536,"ancestor_record_offset":23032,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":432,"record_offset":23404,"parent_record_offset":23324,"ancestor_record_offset":23032,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":432,"record_offset":23372,"parent_record_offset":23324,"ancestor_record_offset":23032,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":23708,"parent_record_offset":23536,"ancestor_record_offset":23032,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":23288,"parent_record_offset":23212,"ancestor_record_offset":23032,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":23212,"parent_record_offset":23176,"ancestor_record_offset":23032,"inlinee":11671,"source_line":696,"archive":"_pdb/C/modules/00432.jsonl.gz"}
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
// 完整结构及原始类型记录: stairs_rope_bridge.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::RopeBridgeNoise
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stairs_rope_bridge.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stairs_rope_bridge.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::RopeBridgeNoise::phase
// 信息 ID: b133138f8ce63860ba8c0c44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve; module=432, inline_record=4136; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve; module=432, inline_record=4680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::RopeBridgeNoise::sample
// 信息 ID: 6dacd83f5ac5b1fbd5db8820
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve; module=432, inline_record=4160; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve; module=432, inline_record=4704; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::ops::range::Range<usize>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure_env$1> > > >; module=1600, inline_record=46096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge
// 信息 ID: 5a5686c28d3393507895b622
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x369bfa -> 0x8491f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37cf4d -> 0x8491f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$0
// 信息 ID: a4bdc77dced93e7574b4dd8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=14984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$1
// 信息 ID: 939e5fcb3fa6b6abc30a4dc5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=16004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$2
// 信息 ID: 832833780a0a9634dc23929a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=16376; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=19076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$2::closure$0
// 信息 ID: 89252133878f0a1f9408ad5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=16576; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=17164; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=19284; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=19864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge::closure$2::closure$0::closure$0
// 信息 ID: e40b306f223113d97547938c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=16656; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=17236; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=19348; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=19928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::densify_rope_curve
// 信息 ID: a63fe0e2d0115097d8d2fb4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=20732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve
// 信息 ID: 326a4c5b73363aeec2269b38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge @ 0x84a684 -> 0x848040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge @ 0x84a706 -> 0x848040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge @ 0x84ac20 -> 0x848040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge @ 0x84ba72 -> 0x848040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge @ 0x84bb0f -> 0x848040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure$0
// 信息 ID: 1c0bc4e5d9b7e185914d1518
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve; module=432, inline_record=1956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure$1
// 信息 ID: ae9718dd588966e6cb2a45e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure_env$1 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::ops::range::Range<usize>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure_env$1 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::ops::range::Range<usize>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::ops::range::Range<usize>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve::closure_env$1> > > >; module=1600, inline_record=45064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::impl$0::default
// 信息 ID: a78c0e9f8da4a8ec99e20b1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x366f02 -> 0x847fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x378f1e -> 0x847fa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::parabolic_rope_panels
// 信息 ID: 27e32b44bbb144602b176d60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=27532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::pull_rope_tips_to_posts
// 信息 ID: 8817863a1e75da8a82cbc095
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge @ 0x84a6ad -> 0x848a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge @ 0x84a72f -> 0x848a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge @ 0x84ac4b -> 0x848a00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::queue_swept_rope
// 信息 ID: efe473e0340fccf91488a498
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=23032; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=35756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_noise_scale
// 信息 ID: 2e5bd1aef7713a54c4793035
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::distort_rope_curve; module=432, inline_record=1984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_post_attachment_target
// 信息 ID: 77d72be1932cbf5d9ebeacf2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=16692; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=17372; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=19480; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=20064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::rope_post_top_offset
// 信息 ID: 35c1be8491595b25cf33a3a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::utils::stairs_rope_bridge::assemble_wooden_rope_bridge; module=432, inline_record=12820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
