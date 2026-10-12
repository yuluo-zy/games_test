// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/dpi_scaling.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1edc434eb4b3bd1cce9b4d0bfbf33f9d368ca6cc1402a71ca3dc31aecf393f52
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dpi_scaling.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 120, 170, 221
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use winit; // namespace candidate
//   package identities: [{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::dpi_scaling::platform::logical_monitor_position
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":221,"record_offset":1304,"rva":5986432,"va":5374695552}
// 编译实例 RVAs: 0x5b5880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: dpi_scaling.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5356830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374695552,"raw_hex":"e8e242aeff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374695673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1beb0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dpi_scaling.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42362223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374695552,"raw_hex":"e86f658602","size":5,"soft_fail":false,"target_va":5417057968,"va":5374695740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41926326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374695552,"raw_hex":"e8b6be7f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5374695877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::dpi::get_monitor_dpi
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1612130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374695552,"raw_hex":"e862991800","size":5,"soft_fail":false,"target_va":5376307728,"va":5374695593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1624709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374695552,"raw_hex":"e885ca1800","size":5,"soft_fail":false,"target_va":5376320352,"va":5374695638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f64::impl$0::round
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f64.rs
//     evidence={"module":221,"record_offset":1928,"parent_record_offset":1304,"ancestor_record_offset":1304,"inlinee":12863,"source_line":101,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_direct] winit::dpi::PhysicalPosition::to_logical
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":221,"record_offset":1684,"parent_record_offset":1304,"ancestor_record_offset":1304,"inlinee":16820,"source_line":273,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":221,"record_offset":1544,"parent_record_offset":1304,"ancestor_record_offset":1304,"inlinee":13617,"source_line":125,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":221,"record_offset":1460,"parent_record_offset":1304,"ancestor_record_offset":1304,"inlinee":16818,"source_line":154,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":221,"record_offset":1724,"parent_record_offset":1684,"ancestor_record_offset":1304,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$106::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":221,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":1304,"inlinee":12892,"source_line":79,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::classify
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":221,"record_offset":1888,"parent_record_offset":1804,"ancestor_record_offset":1304,"inlinee":12859,"source_line":724,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":221,"record_offset":1832,"parent_record_offset":1804,"ancestor_record_offset":1304,"inlinee":12857,"source_line":795,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":221,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":1304,"inlinee":12856,"source_line":761,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":221,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":1304,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::math::round
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":221,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":1304,"inlinee":12865,"source_line":1780,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":221,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":1304,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":221,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":1304,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":221,"record_offset":1596,"parent_record_offset":1568,"ancestor_record_offset":1304,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":221,"record_offset":1624,"parent_record_offset":1568,"ancestor_record_offset":1304,"inlinee":8040,"source_line":1590,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::validate_scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":221,"record_offset":1780,"parent_record_offset":1684,"ancestor_record_offset":1304,"inlinee":12855,"source_line":161,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":221,"record_offset":1568,"parent_record_offset":1544,"ancestor_record_offset":1304,"inlinee":13354,"source_line":211,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":221,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":1304,"inlinee":16819,"source_line":224,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::dpi_scaling::platform::logical_monitor_size
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":221,"record_offset":536,"rva":5986000,"va":5374695120}
// 编译实例 RVAs: 0x5b56d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: dpi_scaling.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5356667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374695120,"raw_hex":"e88543aeff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374695510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1beb0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dpi_scaling.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42362667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374695120,"raw_hex":"e82b678602","size":5,"soft_fail":false,"target_va":5417057968,"va":5374695296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41926793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374695120,"raw_hex":"e889c07f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5374695410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41926169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374695120,"raw_hex":"e819be7f02","size":5,"soft_fail":false,"target_va":5416621632,"va":5374695458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::dpi::get_monitor_dpi
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1612559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374695120,"raw_hex":"e80f9b1800","size":5,"soft_fail":false,"target_va":5376307728,"va":5374695164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1625139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374695120,"raw_hex":"e833cc1800","size":5,"soft_fail":false,"target_va":5376320352,"va":5374695208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f64::impl$0::round
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f64.rs
//     evidence={"module":221,"record_offset":1188,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":12863,"source_line":101,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_direct] winit::dpi::PhysicalSize::to_logical
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":221,"record_offset":892,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":12853,"source_line":431,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":221,"record_offset":688,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":16818,"source_line":154,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":221,"record_offset":772,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":13616,"source_line":118,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":221,"record_offset":1072,"parent_record_offset":892,"ancestor_record_offset":536,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":221,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":536,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::classify
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":221,"record_offset":1036,"parent_record_offset":952,"ancestor_record_offset":536,"inlinee":12859,"source_line":724,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":221,"record_offset":980,"parent_record_offset":952,"ancestor_record_offset":536,"inlinee":12857,"source_line":795,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":221,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":536,"inlinee":12856,"source_line":761,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":221,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":536,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::math::round
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":221,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":536,"inlinee":12865,"source_line":1780,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":221,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":536,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":221,"record_offset":844,"parent_record_offset":804,"ancestor_record_offset":536,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::validate_scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":221,"record_offset":928,"parent_record_offset":892,"ancestor_record_offset":536,"inlinee":12855,"source_line":161,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":221,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":536,"inlinee":16819,"source_line":224,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":221,"record_offset":804,"parent_record_offset":772,"ancestor_record_offset":536,"inlinee":13355,"source_line":185,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::dpi_scaling::platform::window_size_from_logical_windowed_resolution
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B39
// 首条位置: {"module":170,"inlinee":15161}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::dpi_scaling::platform::window_position_from_logical_monitor_coords
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B3A
// 首条位置: {"module":170,"inlinee":15162}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":170,"record_offset":10944,"parent_record_offset":10668,"ancestor_record_offset":10668,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::dpi::impl$12::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":10692,"parent_record_offset":10668,"ancestor_record_offset":10668,"inlinee":15164,"source_line":223,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":170,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":10668,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$106::from
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":170,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":10668,"inlinee":12892,"source_line":79,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::LogicalPosition::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":10992,"parent_record_offset":10968,"ancestor_record_offset":10668,"inlinee":15167,"source_line":202,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":10668,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$37::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":10668,"inlinee":15166,"source_line":580,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$5::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":10796,"parent_record_offset":10716,"ancestor_record_offset":10668,"inlinee":12885,"source_line":140,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::dpi_scaling::platform::logical_size_for_windowed_resolution
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x322F
// 首条位置: {"module":120,"inlinee":12847}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] winit::dpi::PhysicalSize::to_logical
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":15916,"parent_record_offset":15340,"ancestor_record_offset":15340,"inlinee":12853,"source_line":431,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":15372,"parent_record_offset":15340,"ancestor_record_offset":15340,"inlinee":12848,"source_line":568,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":120,"record_offset":16104,"parent_record_offset":15916,"ancestor_record_offset":15340,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":120,"record_offset":16128,"parent_record_offset":16104,"ancestor_record_offset":15340,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::classify
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":120,"record_offset":16068,"parent_record_offset":15980,"ancestor_record_offset":15340,"inlinee":12859,"source_line":724,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":120,"record_offset":16012,"parent_record_offset":15980,"ancestor_record_offset":15340,"inlinee":12857,"source_line":795,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":120,"record_offset":15980,"parent_record_offset":15956,"ancestor_record_offset":15340,"inlinee":12856,"source_line":761,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":120,"record_offset":16036,"parent_record_offset":16012,"ancestor_record_offset":15340,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::math::round
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":120,"record_offset":16316,"parent_record_offset":16292,"ancestor_record_offset":15340,"inlinee":12865,"source_line":1780,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":120,"record_offset":15488,"parent_record_offset":15456,"ancestor_record_offset":15340,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":15740,"parent_record_offset":15552,"ancestor_record_offset":15340,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":15832,"parent_record_offset":15800,"ancestor_record_offset":15340,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":15672,"parent_record_offset":15640,"ancestor_record_offset":15340,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":15696,"parent_record_offset":15672,"ancestor_record_offset":15340,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":15764,"parent_record_offset":15740,"ancestor_record_offset":15340,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":15856,"parent_record_offset":15832,"ancestor_record_offset":15340,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::f64::impl$0::round
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f64.rs
//     evidence={"module":120,"record_offset":16292,"parent_record_offset":16268,"ancestor_record_offset":15340,"inlinee":12863,"source_line":101,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":120,"record_offset":15640,"parent_record_offset":15612,"ancestor_record_offset":15340,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":120,"record_offset":15612,"parent_record_offset":15584,"ancestor_record_offset":15340,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":120,"record_offset":15552,"parent_record_offset":15516,"ancestor_record_offset":15340,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":120,"record_offset":15516,"parent_record_offset":15488,"ancestor_record_offset":15340,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":120,"record_offset":15800,"parent_record_offset":15516,"ancestor_record_offset":15340,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":120,"record_offset":15584,"parent_record_offset":15552,"ancestor_record_offset":15340,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::LogicalSize::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":16216,"parent_record_offset":15916,"ancestor_record_offset":15340,"inlinee":12861,"source_line":360,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":16244,"parent_record_offset":16216,"ancestor_record_offset":15340,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":16268,"parent_record_offset":16244,"ancestor_record_offset":15340,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::validate_scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":15956,"parent_record_offset":15916,"ancestor_record_offset":15340,"inlinee":12855,"source_line":161,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":120,"record_offset":15400,"parent_record_offset":15372,"ancestor_record_offset":15340,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":120,"record_offset":15456,"parent_record_offset":15428,"ancestor_record_offset":15340,"inlinee":12851,"source_line":456,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":15428,"parent_record_offset":15400,"ancestor_record_offset":15340,"inlinee":12850,"source_line":569,"archive":"_pdb/C/modules/00120.jsonl.gz"}
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
// 完整结构及原始类型记录: dpi_scaling.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: dpi_scaling.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: dpi_scaling.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::deferred_wall_history::PendingWallHistory>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::merge_walls::MergeWallsCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::deferred_wall_history::PendingWallHistory>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::merge_walls::MergeWallsCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::deferred_wall_history::PendingWallHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::merge_walls::MergeWallsCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>

// ------------------------------------------------------------------
// 函数: tiny_glade::dpi_scaling::platform::logical_monitor_position
// 信息 ID: 504e718fc85b36e101200828
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::winit_setup::setup_window @ 0x4d11d7 -> 0x5b5880 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::winit_setup::setup_window @ 0x4d11e7 -> 0x5b5880 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::winit_setup::impl$1::from_winit @ 0x5b71dc -> 0x5b5880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::dpi_scaling::platform::logical_monitor_size
// 信息 ID: 55740c4a1ea9ec416304381c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::winit_setup::setup_window @ 0x4d1195 -> 0x5b56d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::winit_setup::impl$1::from_winit @ 0x5b71e8 -> 0x5b56d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::dpi_scaling::platform::logical_size_for_windowed_resolution
// 信息 ID: 1c82543895f348dc31276788
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tiny_glade::winit_event_loop::run_event_loop::closure_env$0>; module=120, inline_record=15340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::dpi_scaling::platform::window_position_from_logical_monitor_coords
// 信息 ID: 30fca9e09784a8242eb17bb5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::winit_setup::setup_window; module=170, inline_record=10668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::dpi_scaling::platform::window_size_from_logical_windowed_resolution
// 信息 ID: 9c5ebbd9b961f8ea34983256
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::winit_setup::setup_window; module=170, inline_record=10400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
