// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_arches/profile.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ba7098ef5524757462bbc86f925266a5e45a90fa6ba67278370b2814103475be
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: profile.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 397, 1417, 1453
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
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
// 源码行: 0, 307
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6006
// 首条位置: {"module":397,"record_offset":7180,"rva":7754496,"va":5376463616}
// 编译实例 RVAs: 0x765300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5419247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376463616,"raw_hex":"e8efb05200","size":5,"soft_fail":false,"target_va":5381883040,"va":5376463788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":397,"record_offset":7560,"parent_record_offset":7180,"ancestor_record_offset":7180,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":397,"record_offset":7592,"parent_record_offset":7180,"ancestor_record_offset":7180,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":397,"record_offset":7352,"parent_record_offset":7180,"ancestor_record_offset":7180,"inlinee":24578,"source_line":308,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":397,"record_offset":7440,"parent_record_offset":7180,"ancestor_record_offset":7180,"inlinee":24579,"source_line":313,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":397,"record_offset":7380,"parent_record_offset":7352,"ancestor_record_offset":7180,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":7676,"parent_record_offset":7592,"ancestor_record_offset":7180,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":7648,"parent_record_offset":7592,"ancestor_record_offset":7180,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":7620,"parent_record_offset":7592,"ancestor_record_offset":7180,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":397,"record_offset":7408,"parent_record_offset":7352,"ancestor_record_offset":7180,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00397.jsonl.gz"}
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
// 首条位置: {"module":397,"record_offset":272,"rva":7751152,"va":5376460272}
// 编译实例 RVAs: 0x7645f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6395684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e8dc689eff","size":5,"soft_fail":false,"target_va":5370067488,"va":5376463167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ba9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4890511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e87160b5ff","size":5,"soft_fail":false,"target_va":5371570656,"va":5376461162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1650020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e89cd2e6ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5376463439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1379500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e854f3eaff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376460983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1379761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e84ff2eaff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376461260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bece40
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4753142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e8f6864800","size":5,"soft_fail":false,"target_va":5381213760,"va":5376460613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11089578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e8aa36a900","size":5,"soft_fail":false,"target_va":5387551024,"va":5376461441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11090638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e8ce3aa900","size":5,"soft_fail":false,"target_va":5387552320,"va":5376461677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8c10
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11091929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e8d93fa900","size":5,"soft_fail":false,"target_va":5387553808,"va":5376461874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f9920
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11095965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e89d4fa900","size":5,"soft_fail":false,"target_va":5387557152,"va":5376461182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f9be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11096784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e8d052a900","size":5,"soft_fail":false,"target_va":5387557856,"va":5376461067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412002f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11121805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e88db4a900","size":5,"soft_fail":false,"target_va":5387584240,"va":5376462430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141202d50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11132859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e8bbdfa900","size":5,"soft_fail":false,"target_va":5387595088,"va":5376462224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141208a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11157197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e8cd3eaa00","size":5,"soft_fail":false,"target_va":5387618816,"va":5376461614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141237ea0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11351788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e8ec36ad00","size":5,"soft_fail":false,"target_va":5387812512,"va":5376460719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: profile.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35386141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e81df31b02","size":5,"soft_fail":false,"target_va":5411847168,"va":5376461022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35078028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e88c3f1702","size":5,"soft_fail":false,"target_va":5411540963,"va":5376462930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35079339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e8ab441702","size":5,"soft_fail":false,"target_va":5411542064,"va":5376462720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t35079463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e827451702","size":5,"soft_fail":false,"target_va":5411542296,"va":5376462828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35079208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e828441702","size":5,"soft_fail":false,"target_va":5411541904,"va":5376462691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35079891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e8d3461702","size":5,"soft_fail":false,"target_va":5411542880,"va":5376462984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopIdx::index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11121659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e8fbb3a900","size":5,"soft_fail":false,"target_va":5387584128,"va":5376462464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11123743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e81fbca900","size":5,"soft_fail":false,"target_va":5387584384,"va":5376460636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11123765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376460272,"raw_hex":"e835bca900","size":5,"soft_fail":false,"target_va":5387584368,"va":5376460598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":397,"record_offset":444,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24415,"source_line":118,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":397,"record_offset":7148,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":397,"record_offset":476,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24571,"source_line":62,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":272,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":272,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":2028,"parent_record_offset":1852,"ancestor_record_offset":272,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":2692,"parent_record_offset":2604,"ancestor_record_offset":272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":272,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":1792,"parent_record_offset":1764,"ancestor_record_offset":272,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":2632,"parent_record_offset":2604,"ancestor_record_offset":272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":272,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":272,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":1852,"parent_record_offset":1820,"ancestor_record_offset":272,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":1820,"parent_record_offset":1792,"ancestor_record_offset":272,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":1888,"parent_record_offset":1852,"ancestor_record_offset":272,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":397,"record_offset":1728,"parent_record_offset":1700,"ancestor_record_offset":272,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":397,"record_offset":1700,"parent_record_offset":1636,"ancestor_record_offset":272,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":6176,"parent_record_offset":6152,"ancestor_record_offset":272,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":1672,"parent_record_offset":1636,"ancestor_record_offset":272,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":3284,"parent_record_offset":3236,"ancestor_record_offset":272,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":1764,"parent_record_offset":1728,"ancestor_record_offset":272,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":1608,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":736,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":272,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":272,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":1636,"parent_record_offset":1608,"ancestor_record_offset":272,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":397,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":272,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":397,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":272,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":397,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":272,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":397,"record_offset":3164,"parent_record_offset":3140,"ancestor_record_offset":272,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":397,"record_offset":3032,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":397,"record_offset":3916,"parent_record_offset":3764,"ancestor_record_offset":272,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":397,"record_offset":7116,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":397,"record_offset":3204,"parent_record_offset":3060,"ancestor_record_offset":272,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":397,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":272,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":397,"record_offset":1036,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":397,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":272,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":397,"record_offset":6416,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":397,"record_offset":1312,"parent_record_offset":1280,"ancestor_record_offset":272,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":397,"record_offset":6356,"parent_record_offset":6072,"ancestor_record_offset":272,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":397,"record_offset":6448,"parent_record_offset":6416,"ancestor_record_offset":272,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":397,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":272,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":397,"record_offset":6480,"parent_record_offset":6448,"ancestor_record_offset":272,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":397,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":272,"inlinee":17977,"source_line":105,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":397,"record_offset":1560,"parent_record_offset":1312,"ancestor_record_offset":272,"inlinee":17981,"source_line":109,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":397,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":272,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":397,"record_offset":2380,"parent_record_offset":2268,"ancestor_record_offset":272,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":397,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":272,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":397,"record_offset":1208,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":397,"record_offset":6072,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":272,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":272,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":2660,"parent_record_offset":2632,"ancestor_record_offset":272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":397,"record_offset":7060,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":19714,"source_line":35,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":4508,"parent_record_offset":4480,"ancestor_record_offset":272,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":4364,"parent_record_offset":4332,"ancestor_record_offset":272,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":1424,"parent_record_offset":1384,"ancestor_record_offset":272,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":397,"record_offset":6816,"parent_record_offset":6784,"ancestor_record_offset":272,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":397,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":272,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":397,"record_offset":3092,"parent_record_offset":3060,"ancestor_record_offset":272,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":397,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":272,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":397,"record_offset":2128,"parent_record_offset":1728,"ancestor_record_offset":272,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":2508,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":397,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":272,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":272,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":1088,"parent_record_offset":1060,"ancestor_record_offset":272,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":397,"record_offset":2228,"parent_record_offset":2200,"ancestor_record_offset":272,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":1124,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":5988,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":397,"record_offset":764,"parent_record_offset":736,"ancestor_record_offset":272,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":397,"record_offset":792,"parent_record_offset":764,"ancestor_record_offset":272,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":397,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":272,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":272,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":397,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":272,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":6692,"parent_record_offset":6668,"ancestor_record_offset":272,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":272,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":6960,"parent_record_offset":6896,"ancestor_record_offset":272,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":3684,"parent_record_offset":3656,"ancestor_record_offset":272,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":6668,"parent_record_offset":6592,"ancestor_record_offset":272,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":397,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":272,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":397,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":272,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":397,"record_offset":4480,"parent_record_offset":4308,"ancestor_record_offset":272,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":272,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":1452,"parent_record_offset":1384,"ancestor_record_offset":272,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":397,"record_offset":1488,"parent_record_offset":1452,"ancestor_record_offset":272,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":397,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":272,"inlinee":24573,"source_line":176,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":397,"record_offset":6844,"parent_record_offset":6816,"ancestor_record_offset":272,"inlinee":24576,"source_line":260,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":2200,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":3236,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":24574,"source_line":851,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":2268,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":24511,"source_line":1147,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":272,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":3976,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":3004,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":4036,"parent_record_offset":4004,"ancestor_record_offset":272,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":4160,"parent_record_offset":4004,"ancestor_record_offset":272,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":397,"record_offset":3060,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":397,"record_offset":4284,"parent_record_offset":4004,"ancestor_record_offset":272,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FFB
// 首条位置: {"module":397,"inlinee":24571}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":1608,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":736,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":397,"record_offset":3032,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":397,"record_offset":7116,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":397,"record_offset":1036,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":397,"record_offset":6416,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":397,"record_offset":5668,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":397,"record_offset":1208,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":397,"record_offset":6072,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":397,"record_offset":7060,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":19714,"source_line":35,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":2508,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":1124,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":5988,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":2200,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":3236,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":24574,"source_line":851,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":2268,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":24511,"source_line":1147,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":3976,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":3004,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":397,"record_offset":3060,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":476,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":476,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":476,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":2028,"parent_record_offset":1852,"ancestor_record_offset":476,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":397,"record_offset":2692,"parent_record_offset":2604,"ancestor_record_offset":476,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":476,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":1792,"parent_record_offset":1764,"ancestor_record_offset":476,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":2632,"parent_record_offset":2604,"ancestor_record_offset":476,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":476,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":476,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":476,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":1852,"parent_record_offset":1820,"ancestor_record_offset":476,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":1820,"parent_record_offset":1792,"ancestor_record_offset":476,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":476,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":397,"record_offset":1888,"parent_record_offset":1852,"ancestor_record_offset":476,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":397,"record_offset":1728,"parent_record_offset":1700,"ancestor_record_offset":476,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":397,"record_offset":1700,"parent_record_offset":1636,"ancestor_record_offset":476,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":6176,"parent_record_offset":6152,"ancestor_record_offset":476,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":1672,"parent_record_offset":1636,"ancestor_record_offset":476,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":3284,"parent_record_offset":3236,"ancestor_record_offset":476,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":1764,"parent_record_offset":1728,"ancestor_record_offset":476,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":4064,"parent_record_offset":4004,"ancestor_record_offset":476,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":476,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":476,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":397,"record_offset":1636,"parent_record_offset":1608,"ancestor_record_offset":476,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":397,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":476,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":397,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":476,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":397,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":476,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":397,"record_offset":3164,"parent_record_offset":3140,"ancestor_record_offset":476,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":397,"record_offset":3916,"parent_record_offset":3764,"ancestor_record_offset":476,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":397,"record_offset":3204,"parent_record_offset":3060,"ancestor_record_offset":476,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":397,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":476,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":397,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":476,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":397,"record_offset":6756,"parent_record_offset":6552,"ancestor_record_offset":476,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":397,"record_offset":1312,"parent_record_offset":1280,"ancestor_record_offset":476,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":397,"record_offset":6356,"parent_record_offset":6072,"ancestor_record_offset":476,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":397,"record_offset":6448,"parent_record_offset":6416,"ancestor_record_offset":476,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":397,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":476,"inlinee":4924,"source_line":155,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":397,"record_offset":6480,"parent_record_offset":6448,"ancestor_record_offset":476,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":397,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":476,"inlinee":17977,"source_line":105,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":397,"record_offset":1560,"parent_record_offset":1312,"ancestor_record_offset":476,"inlinee":17981,"source_line":109,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":397,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":476,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":397,"record_offset":2380,"parent_record_offset":2268,"ancestor_record_offset":476,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":397,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":476,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":476,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":476,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":397,"record_offset":2660,"parent_record_offset":2632,"ancestor_record_offset":476,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":4508,"parent_record_offset":4480,"ancestor_record_offset":476,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":4364,"parent_record_offset":4332,"ancestor_record_offset":476,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":1424,"parent_record_offset":1384,"ancestor_record_offset":476,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":397,"record_offset":6816,"parent_record_offset":6784,"ancestor_record_offset":476,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":397,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":476,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":397,"record_offset":3092,"parent_record_offset":3060,"ancestor_record_offset":476,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":397,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":476,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":397,"record_offset":2128,"parent_record_offset":1728,"ancestor_record_offset":476,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":397,"record_offset":2532,"parent_record_offset":2508,"ancestor_record_offset":476,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":397,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":476,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":476,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":397,"record_offset":1088,"parent_record_offset":1060,"ancestor_record_offset":476,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":397,"record_offset":2228,"parent_record_offset":2200,"ancestor_record_offset":476,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":397,"record_offset":2296,"parent_record_offset":2268,"ancestor_record_offset":476,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":397,"record_offset":764,"parent_record_offset":736,"ancestor_record_offset":476,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":397,"record_offset":792,"parent_record_offset":764,"ancestor_record_offset":476,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":397,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":476,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":476,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":397,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":476,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":6692,"parent_record_offset":6668,"ancestor_record_offset":476,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":476,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":6960,"parent_record_offset":6896,"ancestor_record_offset":476,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":3684,"parent_record_offset":3656,"ancestor_record_offset":476,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":397,"record_offset":6668,"parent_record_offset":6592,"ancestor_record_offset":476,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":397,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":476,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":397,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":476,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":397,"record_offset":4480,"parent_record_offset":4308,"ancestor_record_offset":476,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":476,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":1452,"parent_record_offset":1384,"ancestor_record_offset":476,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":476,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":397,"record_offset":1488,"parent_record_offset":1452,"ancestor_record_offset":476,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":397,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":476,"inlinee":24573,"source_line":176,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":397,"record_offset":6844,"parent_record_offset":6816,"ancestor_record_offset":476,"inlinee":24576,"source_line":260,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":2436,"parent_record_offset":2268,"ancestor_record_offset":476,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":476,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":4036,"parent_record_offset":4004,"ancestor_record_offset":476,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":397,"record_offset":4160,"parent_record_offset":4004,"ancestor_record_offset":476,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":397,"record_offset":4284,"parent_record_offset":4004,"ancestor_record_offset":476,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FFD
// 首条位置: {"module":397,"inlinee":24573}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":1336,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":397,"record_offset":1424,"parent_record_offset":1384,"ancestor_record_offset":1336,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":1452,"parent_record_offset":1384,"ancestor_record_offset":1336,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":397,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":1336,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":397,"record_offset":1488,"parent_record_offset":1452,"ancestor_record_offset":1336,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 234
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114B5
// 首条位置: {"module":1417,"inlinee":70837}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1417,"record_offset":7360,"parent_record_offset":6780,"ancestor_record_offset":6780,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1417,"record_offset":7468,"parent_record_offset":6780,"ancestor_record_offset":6780,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1417,"record_offset":7220,"parent_record_offset":6780,"ancestor_record_offset":6780,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_arches::profile::ArchFunction::remap_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1417,"record_offset":6848,"parent_record_offset":6780,"ancestor_record_offset":6780,"inlinee":24582,"source_line":307,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1417,"record_offset":6948,"parent_record_offset":6920,"ancestor_record_offset":6780,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1417,"record_offset":7068,"parent_record_offset":6848,"ancestor_record_offset":6780,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1417,"record_offset":7184,"parent_record_offset":7100,"ancestor_record_offset":6780,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1417,"record_offset":7156,"parent_record_offset":7100,"ancestor_record_offset":6780,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1417,"record_offset":7128,"parent_record_offset":7100,"ancestor_record_offset":6780,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1417,"record_offset":7392,"parent_record_offset":7360,"ancestor_record_offset":6780,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1417,"record_offset":7424,"parent_record_offset":7392,"ancestor_record_offset":6780,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1417,"record_offset":7100,"parent_record_offset":6848,"ancestor_record_offset":6780,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1417,"record_offset":6976,"parent_record_offset":6920,"ancestor_record_offset":6780,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1417,"record_offset":6920,"parent_record_offset":6848,"ancestor_record_offset":6780,"inlinee":24578,"source_line":308,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_arches/profile.rs
//     evidence={"module":1417,"record_offset":7036,"parent_record_offset":6848,"ancestor_record_offset":6780,"inlinee":24579,"source_line":313,"archive":"_pdb/B/modules/01417.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 260
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6000
// 首条位置: {"module":397,"inlinee":24576}
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
// 源码行: 308
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6002
// 首条位置: {"module":397,"inlinee":24578}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":397,"record_offset":7380,"parent_record_offset":7352,"ancestor_record_offset":7352,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":397,"record_offset":7408,"parent_record_offset":7352,"ancestor_record_offset":7352,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00397.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 313
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6003
// 首条位置: {"module":397,"inlinee":24579}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::ArchFunction::remap_t
// 信息 ID: ff573d68609cd601a84cc281
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4<false>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$20::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4<false> > > > > @ 0x48df7a -> 0x765300 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$20::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; module=1417, inline_record=6848; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile
// 信息 ID: 3303288d50d0941ab281ca2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x760794 -> 0x7645f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl
// 信息 ID: 38e56b8ab1dde67800700d75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_stone_arch_profile; module=397, inline_record=476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$0
// 信息 ID: 33439a60571295669ddf87f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_stone_arch_profile; module=397, inline_record=1336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$1
// 信息 ID: 15532f4cb0fa007420de6b2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$20::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$20::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$20::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; module=1417, inline_record=6780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure$2
// 信息 ID: 64a26946a6ca8c0b014bbc3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_stone_arch_profile; module=397, inline_record=6844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$0
// 信息 ID: 54006cff82861201cfd923cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_arches::profile::impl$0::remap_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::ArchFunction::remap_t; module=397, inline_record=7352; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::ArchFunction::remap_t; module=397, inline_record=7472; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$20::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; module=1417, inline_record=6920; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$20::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; module=1417, inline_record=7008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_arches::profile::impl$0::remap_t::closure$1
// 信息 ID: 81b53b77119f7596809a86ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_arches::profile::impl$0::remap_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::ArchFunction::remap_t; module=397, inline_record=7440; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$20::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env$1> > > >; module=1417, inline_record=7036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
