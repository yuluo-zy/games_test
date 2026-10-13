// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_arches/profile.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 645057207bd6ea27897d23bbdf5d159c46e9c129f2925cc12738cd3ceeda0fd9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: profile.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 405, 1455, 1482
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::ArchFunction::remap_t
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 0, 361
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66C3
// 首条位置: {"module":405,"record_offset":280,"rva":8438448,"va":5377147568}
// 编译实例 RVAs: 0x80c2b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6126223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377147568,"raw_hex":"e88f7a5d00","size":5,"soft_fail":false,"target_va":5383273968,"va":5377147740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":716,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":405,"record_offset":748,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":405,"record_offset":452,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26293,"source_line":362,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":405,"record_offset":568,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26294,"source_line":367,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":480,"parent_record_offset":452,"ancestor_record_offset":280,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":280,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":405,"record_offset":832,"parent_record_offset":748,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":405,"record_offset":804,"parent_record_offset":748,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":405,"record_offset":776,"parent_record_offset":748,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":405,"record_offset":508,"parent_record_offset":452,"ancestor_record_offset":280,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":405,"record_offset":4016,"rva":8439712,"va":5377148832}
// 编译实例 RVAs: 0x80c7a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1470927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e8318ee9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377149610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-1471203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e81d8de9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377149902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8427404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e874687fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377149591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7912324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e87c4487ff","size":5,"soft_fail":false,"target_va":5369240528,"va":5377152847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7911684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e8fc4687ff","size":5,"soft_fail":false,"target_va":5369240608,"va":5377152287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ee920
//     owner=unresolved_namespace; count=2
//     target_aliases=248 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7462975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e8c11f8eff","size":5,"soft_fail":false,"target_va":5369686304,"va":5377149274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140306210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5269833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e8b796afff","size":5,"soft_fail":false,"target_va":5371879952,"va":5377149780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a3e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3574923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e87573c9ff","size":5,"soft_fail":false,"target_va":5373574768,"va":5377149686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3574192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e85076c9ff","size":5,"soft_fail":false,"target_va":5373575632,"va":5377149819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fc850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12515162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e85af7be00","size":5,"soft_fail":false,"target_va":5389666384,"va":5377151217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414054a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12550893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e8ed82bf00","size":5,"soft_fail":false,"target_va":5389702304,"va":5377151406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414054d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12552215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e81788bf00","size":5,"soft_fail":false,"target_va":5389702352,"va":5377150132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12553791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e83f8ebf00","size":5,"soft_fail":false,"target_va":5389704672,"va":5377150876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141473110
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13002555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e83b67c600","size":5,"soft_fail":false,"target_va":5390151952,"va":5377149392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39907410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e852f06002","size":5,"soft_fail":false,"target_va":5417057056,"va":5377149641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39471228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e87c485a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5377149561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t39469626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e83a425a02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377152131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t39470607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e80f465a02","size":5,"soft_fail":false,"target_va":5416622240,"va":5377151628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t39469939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e873435a02","size":5,"soft_fail":false,"target_va":5416621632,"va":5377151688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopIdx::index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12550171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e81b80bf00","size":5,"soft_fail":false,"target_va":5389701616,"va":5377151440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12552419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e8e388bf00","size":5,"soft_fail":false,"target_va":5389701712,"va":5377149288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12552480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377148832,"raw_hex":"e82089bf00","size":5,"soft_fail":false,"target_va":5389701744,"va":5377149259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":405,"record_offset":4184,"parent_record_offset":4016,"ancestor_record_offset":4016,"inlinee":26107,"source_line":149,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":405,"record_offset":4220,"parent_record_offset":4016,"ancestor_record_offset":4016,"inlinee":26300,"source_line":116,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":5876,"parent_record_offset":5852,"ancestor_record_offset":4016,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":5900,"parent_record_offset":5876,"ancestor_record_offset":4016,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":6572,"parent_record_offset":6548,"ancestor_record_offset":4016,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":4016,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":5924,"parent_record_offset":5900,"ancestor_record_offset":4016,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":6620,"parent_record_offset":6596,"ancestor_record_offset":4016,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":5852,"parent_record_offset":5668,"ancestor_record_offset":4016,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":6548,"parent_record_offset":6460,"ancestor_record_offset":4016,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":5612,"parent_record_offset":5588,"ancestor_record_offset":4016,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":6488,"parent_record_offset":6460,"ancestor_record_offset":4016,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":6460,"parent_record_offset":6436,"ancestor_record_offset":4016,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":5668,"parent_record_offset":5636,"ancestor_record_offset":4016,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":4016,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":4016,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":5708,"parent_record_offset":5668,"ancestor_record_offset":4016,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":405,"record_offset":5552,"parent_record_offset":5488,"ancestor_record_offset":4016,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":5524,"parent_record_offset":5488,"ancestor_record_offset":4016,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":7340,"parent_record_offset":7296,"ancestor_record_offset":4016,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":5588,"parent_record_offset":5552,"ancestor_record_offset":4016,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":5460,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":4524,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":6256,"parent_record_offset":6232,"ancestor_record_offset":4016,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":11608,"parent_record_offset":11584,"ancestor_record_offset":4016,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":5488,"parent_record_offset":5460,"ancestor_record_offset":4016,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":405,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":4016,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":405,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":4016,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":405,"record_offset":7252,"parent_record_offset":7228,"ancestor_record_offset":4016,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":7000,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":12360,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":4016,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":405,"record_offset":5812,"parent_record_offset":5756,"ancestor_record_offset":4016,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":405,"record_offset":4836,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":405,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":4016,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":405,"record_offset":11780,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":405,"record_offset":5136,"parent_record_offset":5104,"ancestor_record_offset":4016,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":11524,"parent_record_offset":11500,"ancestor_record_offset":4016,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":11812,"parent_record_offset":11780,"ancestor_record_offset":4016,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":11548,"parent_record_offset":11524,"ancestor_record_offset":4016,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":11844,"parent_record_offset":11812,"ancestor_record_offset":4016,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":405,"record_offset":5056,"parent_record_offset":5032,"ancestor_record_offset":4016,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":405,"record_offset":5412,"parent_record_offset":5136,"ancestor_record_offset":4016,"inlinee":19983,"source_line":164,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":405,"record_offset":11088,"parent_record_offset":11064,"ancestor_record_offset":4016,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":6232,"parent_record_offset":6116,"ancestor_record_offset":4016,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":4016,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":5032,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":11500,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":405,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":4016,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":405,"record_offset":5784,"parent_record_offset":5756,"ancestor_record_offset":4016,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":405,"record_offset":6516,"parent_record_offset":6488,"ancestor_record_offset":4016,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":405,"record_offset":12304,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":405,"record_offset":9656,"parent_record_offset":9628,"ancestor_record_offset":4016,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":405,"record_offset":9512,"parent_record_offset":9480,"ancestor_record_offset":4016,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":405,"record_offset":5248,"parent_record_offset":5208,"ancestor_record_offset":4016,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":405,"record_offset":12120,"parent_record_offset":12088,"ancestor_record_offset":4016,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":405,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":4016,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":405,"record_offset":7088,"parent_record_offset":7056,"ancestor_record_offset":4016,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":405,"record_offset":12088,"parent_record_offset":12060,"ancestor_record_offset":4016,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":405,"record_offset":5980,"parent_record_offset":5552,"ancestor_record_offset":4016,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":405,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":4016,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":405,"record_offset":6364,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":405,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":4016,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":405,"record_offset":4912,"parent_record_offset":4876,"ancestor_record_offset":4016,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":405,"record_offset":7812,"parent_record_offset":7772,"ancestor_record_offset":4016,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":405,"record_offset":6076,"parent_record_offset":6048,"ancestor_record_offset":4016,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":405,"record_offset":4948,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":405,"record_offset":11416,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":405,"record_offset":4552,"parent_record_offset":4524,"ancestor_record_offset":4016,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":405,"record_offset":4580,"parent_record_offset":4552,"ancestor_record_offset":4016,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":405,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":4016,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":405,"record_offset":11440,"parent_record_offset":11416,"ancestor_record_offset":4016,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":405,"record_offset":4876,"parent_record_offset":4836,"ancestor_record_offset":4016,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":405,"record_offset":7772,"parent_record_offset":7732,"ancestor_record_offset":4016,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":405,"record_offset":9456,"parent_record_offset":9432,"ancestor_record_offset":4016,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":405,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":4016,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":405,"record_offset":9628,"parent_record_offset":9456,"ancestor_record_offset":4016,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":405,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":4016,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":405,"record_offset":5276,"parent_record_offset":5208,"ancestor_record_offset":4016,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":405,"record_offset":5208,"parent_record_offset":5184,"ancestor_record_offset":4016,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":405,"record_offset":5312,"parent_record_offset":5276,"ancestor_record_offset":4016,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":405,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":4016,"inlinee":26302,"source_line":230,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":405,"record_offset":12148,"parent_record_offset":12120,"ancestor_record_offset":4016,"inlinee":26305,"source_line":314,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":6048,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_slice
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":7296,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":26303,"source_line":851,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":6116,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":26086,"source_line":1147,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":9152,"parent_record_offset":9124,"ancestor_record_offset":4016,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":9124,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":6972,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":9184,"parent_record_offset":9152,"ancestor_record_offset":4016,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":9308,"parent_record_offset":9152,"ancestor_record_offset":4016,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":405,"record_offset":7056,"parent_record_offset":4220,"ancestor_record_offset":4016,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":405,"record_offset":9432,"parent_record_offset":9152,"ancestor_record_offset":4016,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_wood_arch_profile
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":405,"record_offset":868,"rva":8438704,"va":5377147824}
// 编译实例 RVAs: 0x80c3b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1469368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377147824,"raw_hex":"e84894e9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377148051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1469778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377147824,"raw_hex":"e8ae92e9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377148477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8425851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377147824,"raw_hex":"e8856e7fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377148038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414054a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12553660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377147824,"raw_hex":"e8bc8dbf00","size":5,"soft_fail":false,"target_va":5389702304,"va":5377148639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39471972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377147824,"raw_hex":"e8644b5a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5377148817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39472953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377147824,"raw_hex":"e8394f5a02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377148804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopIdx::index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12552942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377147824,"raw_hex":"e8ee8abf00","size":5,"soft_fail":false,"target_va":5389701616,"va":5377148669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":3956,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":3984,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":405,"record_offset":3376,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":1824,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":1200,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":1060,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":405,"record_offset":3900,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":405,"record_offset":2720,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":405,"record_offset":3012,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":868,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":868,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":2908,"parent_record_offset":2884,"ancestor_record_offset":868,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":868,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":868,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":868,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":2104,"parent_record_offset":2072,"ancestor_record_offset":868,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":2884,"parent_record_offset":2792,"ancestor_record_offset":868,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":2012,"parent_record_offset":1984,"ancestor_record_offset":868,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":2820,"parent_record_offset":2792,"ancestor_record_offset":868,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":868,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":2072,"parent_record_offset":2040,"ancestor_record_offset":868,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":868,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":868,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":2260,"parent_record_offset":2236,"ancestor_record_offset":868,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":1956,"parent_record_offset":1920,"ancestor_record_offset":868,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":1984,"parent_record_offset":1956,"ancestor_record_offset":868,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":1856,"parent_record_offset":1824,"ancestor_record_offset":868,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":868,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":2536,"parent_record_offset":2492,"ancestor_record_offset":868,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":405,"record_offset":2640,"parent_record_offset":2536,"ancestor_record_offset":868,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":405,"record_offset":2236,"parent_record_offset":1920,"ancestor_record_offset":868,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":405,"record_offset":1888,"parent_record_offset":1856,"ancestor_record_offset":868,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":405,"record_offset":1920,"parent_record_offset":1888,"ancestor_record_offset":868,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":405,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":868,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":405,"record_offset":1764,"parent_record_offset":1652,"ancestor_record_offset":868,"inlinee":16325,"source_line":499,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":405,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":868,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":405,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":868,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":405,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":868,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":868,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":405,"record_offset":1416,"parent_record_offset":1224,"ancestor_record_offset":868,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":405,"record_offset":1252,"parent_record_offset":1224,"ancestor_record_offset":868,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":405,"record_offset":1576,"parent_record_offset":1464,"ancestor_record_offset":868,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":868,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":868,"inlinee":9710,"source_line":99,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":868,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":868,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":3440,"parent_record_offset":3408,"ancestor_record_offset":868,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":1464,"parent_record_offset":1440,"ancestor_record_offset":868,"inlinee":9711,"source_line":275,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":868,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":868,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":405,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":868,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":405,"record_offset":1492,"parent_record_offset":1464,"ancestor_record_offset":868,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":2284,"parent_record_offset":2260,"ancestor_record_offset":868,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":2492,"parent_record_offset":2412,"ancestor_record_offset":868,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":1628,"parent_record_offset":1576,"ancestor_record_offset":868,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":868,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":405,"record_offset":2852,"parent_record_offset":2820,"ancestor_record_offset":868,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":405,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":868,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":405,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":868,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":405,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":868,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":405,"record_offset":1332,"parent_record_offset":1252,"ancestor_record_offset":868,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":405,"record_offset":2744,"parent_record_offset":2720,"ancestor_record_offset":868,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":405,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":868,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":405,"record_offset":2380,"parent_record_offset":2332,"ancestor_record_offset":868,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":405,"record_offset":3592,"parent_record_offset":3552,"ancestor_record_offset":868,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":405,"record_offset":2592,"parent_record_offset":2536,"ancestor_record_offset":868,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":405,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":868,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":405,"record_offset":3252,"parent_record_offset":3228,"ancestor_record_offset":868,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":405,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":868,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":405,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":868,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":405,"record_offset":3796,"parent_record_offset":3440,"ancestor_record_offset":868,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":405,"record_offset":3552,"parent_record_offset":3512,"ancestor_record_offset":868,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_wood_arch_profile::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":405,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":868,"inlinee":26296,"source_line":73,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_wood_arch_profile::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":405,"record_offset":2444,"parent_record_offset":2412,"ancestor_record_offset":868,"inlinee":26297,"source_line":85,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_wood_arch_profile::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":405,"record_offset":3744,"parent_record_offset":3716,"ancestor_record_offset":868,"inlinee":26298,"source_line":95,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_wood_arch_profile::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66B8
// 首条位置: {"module":405,"inlinee":26296}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_wood_arch_profile::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66B9
// 首条位置: {"module":405,"inlinee":26297}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_wood_arch_profile::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66BA
// 首条位置: {"module":405,"inlinee":26298}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66BC
// 首条位置: {"module":405,"inlinee":26300}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":5460,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":4524,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":7000,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":12360,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":405,"record_offset":4836,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":405,"record_offset":11780,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":11040,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":5032,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":11500,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":405,"record_offset":12304,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":405,"record_offset":6364,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":405,"record_offset":4948,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":405,"record_offset":11416,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":6048,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_slice
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":7296,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":26303,"source_line":851,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":6116,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":26086,"source_line":1147,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":9124,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":6972,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":405,"record_offset":7056,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":5876,"parent_record_offset":5852,"ancestor_record_offset":4220,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":5900,"parent_record_offset":5876,"ancestor_record_offset":4220,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":6572,"parent_record_offset":6548,"ancestor_record_offset":4220,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":4220,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":5924,"parent_record_offset":5900,"ancestor_record_offset":4220,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":6620,"parent_record_offset":6596,"ancestor_record_offset":4220,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":5852,"parent_record_offset":5668,"ancestor_record_offset":4220,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":405,"record_offset":6548,"parent_record_offset":6460,"ancestor_record_offset":4220,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":5612,"parent_record_offset":5588,"ancestor_record_offset":4220,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":6488,"parent_record_offset":6460,"ancestor_record_offset":4220,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":6460,"parent_record_offset":6436,"ancestor_record_offset":4220,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":5668,"parent_record_offset":5636,"ancestor_record_offset":4220,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":4220,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":4220,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":405,"record_offset":5708,"parent_record_offset":5668,"ancestor_record_offset":4220,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":405,"record_offset":5552,"parent_record_offset":5488,"ancestor_record_offset":4220,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":5524,"parent_record_offset":5488,"ancestor_record_offset":4220,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":7340,"parent_record_offset":7296,"ancestor_record_offset":4220,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":5588,"parent_record_offset":5552,"ancestor_record_offset":4220,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":9212,"parent_record_offset":9152,"ancestor_record_offset":4220,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":6256,"parent_record_offset":6232,"ancestor_record_offset":4220,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":11608,"parent_record_offset":11584,"ancestor_record_offset":4220,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":405,"record_offset":5488,"parent_record_offset":5460,"ancestor_record_offset":4220,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":405,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":4220,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":405,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":4220,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":405,"record_offset":7252,"parent_record_offset":7228,"ancestor_record_offset":4220,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":4220,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":405,"record_offset":5812,"parent_record_offset":5756,"ancestor_record_offset":4220,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":405,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":4220,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":405,"record_offset":12060,"parent_record_offset":11916,"ancestor_record_offset":4220,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":405,"record_offset":5136,"parent_record_offset":5104,"ancestor_record_offset":4220,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":11524,"parent_record_offset":11500,"ancestor_record_offset":4220,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":11812,"parent_record_offset":11780,"ancestor_record_offset":4220,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":11548,"parent_record_offset":11524,"ancestor_record_offset":4220,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":405,"record_offset":11844,"parent_record_offset":11812,"ancestor_record_offset":4220,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":405,"record_offset":5056,"parent_record_offset":5032,"ancestor_record_offset":4220,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":405,"record_offset":5412,"parent_record_offset":5136,"ancestor_record_offset":4220,"inlinee":19983,"source_line":164,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":405,"record_offset":11088,"parent_record_offset":11064,"ancestor_record_offset":4220,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":6232,"parent_record_offset":6116,"ancestor_record_offset":4220,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":405,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":4220,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":405,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":4220,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":405,"record_offset":5784,"parent_record_offset":5756,"ancestor_record_offset":4220,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":405,"record_offset":6516,"parent_record_offset":6488,"ancestor_record_offset":4220,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":405,"record_offset":9656,"parent_record_offset":9628,"ancestor_record_offset":4220,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":405,"record_offset":9512,"parent_record_offset":9480,"ancestor_record_offset":4220,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":405,"record_offset":5248,"parent_record_offset":5208,"ancestor_record_offset":4220,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":405,"record_offset":12120,"parent_record_offset":12088,"ancestor_record_offset":4220,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":405,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":4220,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":405,"record_offset":7088,"parent_record_offset":7056,"ancestor_record_offset":4220,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":405,"record_offset":12088,"parent_record_offset":12060,"ancestor_record_offset":4220,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":405,"record_offset":5980,"parent_record_offset":5552,"ancestor_record_offset":4220,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":405,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":4220,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":405,"record_offset":6388,"parent_record_offset":6364,"ancestor_record_offset":4220,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":405,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":4220,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":405,"record_offset":4912,"parent_record_offset":4876,"ancestor_record_offset":4220,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":405,"record_offset":7812,"parent_record_offset":7772,"ancestor_record_offset":4220,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":405,"record_offset":6076,"parent_record_offset":6048,"ancestor_record_offset":4220,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":405,"record_offset":6148,"parent_record_offset":6116,"ancestor_record_offset":4220,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":405,"record_offset":4552,"parent_record_offset":4524,"ancestor_record_offset":4220,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":405,"record_offset":4580,"parent_record_offset":4552,"ancestor_record_offset":4220,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":405,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":4220,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":405,"record_offset":11440,"parent_record_offset":11416,"ancestor_record_offset":4220,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":405,"record_offset":4876,"parent_record_offset":4836,"ancestor_record_offset":4220,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":405,"record_offset":7772,"parent_record_offset":7732,"ancestor_record_offset":4220,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":405,"record_offset":9456,"parent_record_offset":9432,"ancestor_record_offset":4220,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":405,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":4220,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":405,"record_offset":9628,"parent_record_offset":9456,"ancestor_record_offset":4220,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":405,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":4220,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":405,"record_offset":5276,"parent_record_offset":5208,"ancestor_record_offset":4220,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":405,"record_offset":5208,"parent_record_offset":5184,"ancestor_record_offset":4220,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":405,"record_offset":5312,"parent_record_offset":5276,"ancestor_record_offset":4220,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":405,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":4220,"inlinee":26302,"source_line":230,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":405,"record_offset":12148,"parent_record_offset":12120,"ancestor_record_offset":4220,"inlinee":26305,"source_line":314,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":6288,"parent_record_offset":6116,"ancestor_record_offset":4220,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":9152,"parent_record_offset":9124,"ancestor_record_offset":4220,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":9184,"parent_record_offset":9152,"ancestor_record_offset":4220,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":405,"record_offset":9308,"parent_record_offset":9152,"ancestor_record_offset":4220,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":405,"record_offset":9432,"parent_record_offset":9152,"ancestor_record_offset":4220,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 230
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66BE
// 首条位置: {"module":405,"inlinee":26302}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":405,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":5160,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":5160,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":405,"record_offset":5248,"parent_record_offset":5208,"ancestor_record_offset":5160,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":405,"record_offset":5276,"parent_record_offset":5208,"ancestor_record_offset":5160,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":405,"record_offset":5208,"parent_record_offset":5184,"ancestor_record_offset":5160,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":405,"record_offset":5312,"parent_record_offset":5276,"ancestor_record_offset":5160,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 288
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13715
// 首条位置: {"module":1482,"inlinee":79637}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1482,"record_offset":34256,"parent_record_offset":33648,"ancestor_record_offset":33648,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1482,"record_offset":34364,"parent_record_offset":33648,"ancestor_record_offset":33648,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1482,"record_offset":34116,"parent_record_offset":33648,"ancestor_record_offset":33648,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_arches::profile::ArchFunction::remap_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1482,"record_offset":33716,"parent_record_offset":33648,"ancestor_record_offset":33648,"inlinee":26307,"source_line":361,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1482,"record_offset":33816,"parent_record_offset":33788,"ancestor_record_offset":33648,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1482,"record_offset":33964,"parent_record_offset":33716,"ancestor_record_offset":33648,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1482,"record_offset":33868,"parent_record_offset":33844,"ancestor_record_offset":33648,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1482,"record_offset":34080,"parent_record_offset":33996,"ancestor_record_offset":33648,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1482,"record_offset":34052,"parent_record_offset":33996,"ancestor_record_offset":33648,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1482,"record_offset":34024,"parent_record_offset":33996,"ancestor_record_offset":33648,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1482,"record_offset":34288,"parent_record_offset":34256,"ancestor_record_offset":33648,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1482,"record_offset":34320,"parent_record_offset":34288,"ancestor_record_offset":33648,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1482,"record_offset":33996,"parent_record_offset":33716,"ancestor_record_offset":33648,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1482,"record_offset":33844,"parent_record_offset":33788,"ancestor_record_offset":33648,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1482,"record_offset":33788,"parent_record_offset":33716,"ancestor_record_offset":33648,"inlinee":26293,"source_line":362,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1482,"record_offset":33932,"parent_record_offset":33716,"ancestor_record_offset":33648,"inlinee":26294,"source_line":367,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 314
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66C1
// 首条位置: {"module":405,"inlinee":26305}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 362
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66B5
// 首条位置: {"module":405,"inlinee":26293}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":480,"parent_record_offset":452,"ancestor_record_offset":452,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":405,"record_offset":508,"parent_record_offset":452,"ancestor_record_offset":452,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":405,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":452,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00405.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 367
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66B6
// 首条位置: {"module":405,"inlinee":26294}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: profile.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_arches::profile::ArchFunction
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_arches::profile::ArchProfileRangeInsideSegment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_wall_constructor::construct_arches::profile::ArchProfileRangeInsideSegment,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_wall_constructor::construct_arches::profile::ArchProfileRangeInsideSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: profile.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_arches/profile.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_arches/profile.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_arches/profile.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: profile.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::ArchFunction::remap_t
// 信息 ID: 22b9b74d4d70b9f9c485a2ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4<false>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$21::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4<false> > > > > @ 0x5e029a -> 0x80c2b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$21::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; module=1482, inline_record=33716; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile
// 信息 ID: 5942dbe83931209bc87fa7a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ec219 -> 0x80c7a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl
// 信息 ID: fea68c2f513d5fcdc34a5dba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_stone_arch_profile; module=405, inline_record=4220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$0
// 信息 ID: 125d1cf29588503005b2515a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_stone_arch_profile; module=405, inline_record=5160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$1
// 信息 ID: 68df4c0acf3d830a2d7c7a23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$21::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$21::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$21::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; module=1482, inline_record=33648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$2
// 信息 ID: 9b18745e7a0ccb59b9461880
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_stone_arch_profile; module=405, inline_record=12148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_wood_arch_profile
// 信息 ID: 5130bd0f0a9ac54f218dfb7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ec1b3 -> 0x80c3b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_wood_arch_profile::closure$0
// 信息 ID: 57141b2894f62be6d757992a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_arches::profile::create_wood_arch_profile", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_wood_arch_profile; module=405, inline_record=1380; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_wood_arch_profile; module=405, inline_record=1600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_wood_arch_profile::closure$1
// 信息 ID: ba67372b7aaed13bb8677cf7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_arches::profile::create_wood_arch_profile", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_wood_arch_profile; module=405, inline_record=2444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_wood_arch_profile::closure$2
// 信息 ID: f9481ac4f32d802828f1bcff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_arches::profile::create_wood_arch_profile", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_wood_arch_profile; module=405, inline_record=3744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$0
// 信息 ID: d792f184fea0ce5b7ae86993
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_arches::profile::impl$0::remap_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::ArchFunction::remap_t; module=405, inline_record=452; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::ArchFunction::remap_t; module=405, inline_record=600; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$21::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; module=1482, inline_record=33788; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$21::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; module=1482, inline_record=33904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$1
// 信息 ID: b3dd398a2a241fcbe4bad2c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_arches::profile::impl$0::remap_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::ArchFunction::remap_t; module=405, inline_record=568; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$21::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; module=1482, inline_record=33932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
