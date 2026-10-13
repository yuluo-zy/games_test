// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/merge_proposal_wall_decorators.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: bffb6a4b775ed649960bd36f81d3b46e0abf227d79fb2a758f1b20190078f93f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: merge_proposal_wall_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3185, 3205, 3206
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3205,"record_offset":1308,"rva":38841728,"va":5407550848}
// 编译实例 RVAs: 0x250ad80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-31873687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e869a519fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5407552386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140104100
//     owner=unresolved_namespace; count=1
//     target_aliases=583 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-37777673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e8f78ebffd","size":5,"soft_fail":false,"target_va":5369774336,"va":5407552004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34071267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e81d1df8fd","size":5,"soft_fail":false,"target_va":5373479776,"va":5407551038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=2
//     target_aliases=20 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29424288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e860053ffe","size":5,"soft_fail":false,"target_va":5378126944,"va":5407551227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de62e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24268526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e812b18dfe","size":5,"soft_fail":false,"target_va":5383283424,"va":5407551945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de6350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24268239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e831b28dfe","size":5,"soft_fail":false,"target_va":5383283536,"va":5407551770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de63a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24268263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e819b28dfe","size":5,"soft_fail":false,"target_va":5383283616,"va":5407551874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a11e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17210690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e8be62f9fe","size":5,"soft_fail":false,"target_va":5390340576,"va":5407551261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424f2100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-102801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e86f6efeff","size":5,"soft_fail":false,"target_va":5407449344,"va":5407552140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9070137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e839668a00","size":5,"soft_fail":false,"target_va":5416621536,"va":5407551394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9069770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e8ca648a00","size":5,"soft_fail":false,"target_va":5416622208,"va":5407552433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9069764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e8c4648a00","size":5,"soft_fail":false,"target_va":5416622240,"va":5407552471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29424855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e829033ffe","size":5,"soft_fail":false,"target_va":5378126928,"va":5407551778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::max_rank
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29424381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e803053ffe","size":5,"soft_fail":false,"target_va":5378127056,"va":5407551432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25900381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e8a3ca74fe","size":5,"soft_fail":false,"target_va":5381650864,"va":5407551240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25896886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e84ad874fe","size":5,"soft_fail":false,"target_va":5381654320,"va":5407551201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27335541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407550848,"raw_hex":"e88be45efe","size":5,"soft_fail":false,"target_va":5380216256,"va":5407551792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3205,"record_offset":3888,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3205,"record_offset":3860,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3205,"record_offset":3568,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3205,"record_offset":3140,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3205,"record_offset":3916,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3205,"record_offset":2532,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":27083,"source_line":285,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3205,"record_offset":3628,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3205,"record_offset":3712,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3205,"record_offset":3804,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3205,"record_offset":1544,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::impl$0::snapping_direction
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":3205,"record_offset":3168,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":117944,"source_line":64,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::try_merge_decorators
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":3205,"record_offset":2672,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":117927,"source_line":78,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$0::classify_side
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3205,"record_offset":3684,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":118032,"source_line":43,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3205,"record_offset":3768,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":27690,"source_line":213,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3205,"record_offset":3420,"parent_record_offset":1308,"ancestor_record_offset":1308,"inlinee":27088,"source_line":70,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3205,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":1308,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3205,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":1308,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3205,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":1308,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3205,"record_offset":4100,"parent_record_offset":4000,"ancestor_record_offset":1308,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3205,"record_offset":4032,"parent_record_offset":4000,"ancestor_record_offset":1308,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3205,"record_offset":4000,"parent_record_offset":3972,"ancestor_record_offset":1308,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3205,"record_offset":3972,"parent_record_offset":3944,"ancestor_record_offset":1308,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3205,"record_offset":2856,"parent_record_offset":2828,"ancestor_record_offset":1308,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3205,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":1308,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3205,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":1308,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3205,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":1308,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3205,"record_offset":3500,"parent_record_offset":3476,"ancestor_record_offset":1308,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3205,"record_offset":2356,"parent_record_offset":1720,"ancestor_record_offset":1308,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3205,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":1308,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3205,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":1308,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":3205,"record_offset":2744,"parent_record_offset":2716,"ancestor_record_offset":1308,"inlinee":7944,"source_line":104,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":3205,"record_offset":2964,"parent_record_offset":2940,"ancestor_record_offset":1308,"inlinee":8405,"source_line":49,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":3205,"record_offset":3088,"parent_record_offset":3036,"ancestor_record_offset":1308,"inlinee":8441,"source_line":53,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3205,"record_offset":2716,"parent_record_offset":2672,"ancestor_record_offset":1308,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3205,"record_offset":2800,"parent_record_offset":2772,"ancestor_record_offset":1308,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3205,"record_offset":2940,"parent_record_offset":2672,"ancestor_record_offset":1308,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3205,"record_offset":2772,"parent_record_offset":2744,"ancestor_record_offset":1308,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3205,"record_offset":4068,"parent_record_offset":4032,"ancestor_record_offset":1308,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":3205,"record_offset":2088,"parent_record_offset":2060,"ancestor_record_offset":1308,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":3205,"record_offset":2060,"parent_record_offset":2032,"ancestor_record_offset":1308,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3205,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":1308,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3205,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":1308,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3205,"record_offset":2600,"parent_record_offset":2568,"ancestor_record_offset":1308,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3205,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":1308,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3205,"record_offset":3944,"parent_record_offset":3916,"ancestor_record_offset":1308,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3205,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":1308,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3205,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":1308,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$26::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3205,"record_offset":2884,"parent_record_offset":2856,"ancestor_record_offset":1308,"inlinee":26794,"source_line":21,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3205,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":1308,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3205,"record_offset":3328,"parent_record_offset":3228,"ancestor_record_offset":1308,"inlinee":27086,"source_line":118,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3205,"record_offset":2568,"parent_record_offset":2532,"ancestor_record_offset":1308,"inlinee":27084,"source_line":241,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3205,"record_offset":2632,"parent_record_offset":2600,"ancestor_record_offset":1308,"inlinee":27106,"source_line":244,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3205,"record_offset":2000,"parent_record_offset":1972,"ancestor_record_offset":1308,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3205,"record_offset":2032,"parent_record_offset":2000,"ancestor_record_offset":1308,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3205,"record_offset":1972,"parent_record_offset":1720,"ancestor_record_offset":1308,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3205,"record_offset":1832,"parent_record_offset":1720,"ancestor_record_offset":1308,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3205,"record_offset":2384,"parent_record_offset":1720,"ancestor_record_offset":1308,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3205,"record_offset":1916,"parent_record_offset":1720,"ancestor_record_offset":1308,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":3205,"record_offset":1776,"parent_record_offset":1720,"ancestor_record_offset":1308,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":3205,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":1308,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3205,"record_offset":1612,"parent_record_offset":1576,"ancestor_record_offset":1308,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3205,"record_offset":2240,"parent_record_offset":2132,"ancestor_record_offset":1308,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3205,"record_offset":1644,"parent_record_offset":1612,"ancestor_record_offset":1308,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3205,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":1308,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3205,"record_offset":2476,"parent_record_offset":1720,"ancestor_record_offset":1308,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3205,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":1308,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3205,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":1308,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3205,"record_offset":1672,"parent_record_offset":1612,"ancestor_record_offset":1308,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3205,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":1308,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3205,"record_offset":1804,"parent_record_offset":1720,"ancestor_record_offset":1308,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3205,"record_offset":2132,"parent_record_offset":1720,"ancestor_record_offset":1308,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3205,"record_offset":1576,"parent_record_offset":1544,"ancestor_record_offset":1308,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_wall_decorators.rs
//     evidence={"module":3205,"record_offset":3596,"parent_record_offset":3568,"ancestor_record_offset":1308,"inlinee":118031,"source_line":110,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":3205,"record_offset":2828,"parent_record_offset":2800,"ancestor_record_offset":1308,"inlinee":117928,"source_line":85,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":3205,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":1308,"inlinee":80648,"source_line":90,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3205,"record_offset":3452,"parent_record_offset":3420,"ancestor_record_offset":1308,"inlinee":27090,"source_line":39,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3205,"record_offset":3476,"parent_record_offset":3452,"ancestor_record_offset":1308,"inlinee":27092,"source_line":58,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3205,"record_offset":3380,"parent_record_offset":3328,"ancestor_record_offset":1308,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3205,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":1308,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3205,"record_offset":272,"rva":38840432,"va":5407549552}
// 编译实例 RVAs: 0x250a870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9070759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407549552,"raw_hex":"e8a7688a00","size":5,"soft_fail":false,"target_va":5416621600,"va":5407550836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407549552,"raw_hex":"e828040000","size":5,"soft_fail":false,"target_va":5407550848,"va":5407549779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3205,"record_offset":536,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3205,"record_offset":452,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3205,"record_offset":1184,"parent_record_offset":1120,"ancestor_record_offset":272,"inlinee":11574,"source_line":1957,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3205,"record_offset":1056,"parent_record_offset":1028,"ancestor_record_offset":272,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3205,"record_offset":960,"parent_record_offset":880,"ancestor_record_offset":272,"inlinee":17049,"source_line":39,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3205,"record_offset":848,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3205,"record_offset":604,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3205,"record_offset":1028,"parent_record_offset":960,"ancestor_record_offset":272,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3205,"record_offset":564,"parent_record_offset":536,"ancestor_record_offset":272,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3205,"record_offset":1232,"parent_record_offset":880,"ancestor_record_offset":272,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3205,"record_offset":752,"parent_record_offset":628,"ancestor_record_offset":272,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3205,"record_offset":1092,"parent_record_offset":1056,"ancestor_record_offset":272,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3205,"record_offset":1148,"parent_record_offset":1120,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3205,"record_offset":820,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3205,"record_offset":1268,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3205,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3205,"record_offset":720,"parent_record_offset":660,"ancestor_record_offset":272,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3205,"record_offset":692,"parent_record_offset":660,"ancestor_record_offset":272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3205,"record_offset":476,"parent_record_offset":452,"ancestor_record_offset":272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3205,"record_offset":628,"parent_record_offset":604,"ancestor_record_offset":272,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3205,"record_offset":880,"parent_record_offset":848,"ancestor_record_offset":272,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3205,"record_offset":660,"parent_record_offset":628,"ancestor_record_offset":272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_wall_decorators.rs
//     evidence={"module":3205,"record_offset":780,"parent_record_offset":752,"ancestor_record_offset":272,"inlinee":118029,"source_line":51,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_wall_decorators.rs
//     evidence={"module":3205,"record_offset":1120,"parent_record_offset":1092,"ancestor_record_offset":272,"inlinee":118030,"source_line":65,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CD0D
// 首条位置: {"module":3205,"inlinee":118029}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CD0E
// 首条位置: {"module":3205,"inlinee":118030}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3205,"record_offset":1184,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":11574,"source_line":1957,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3205,"record_offset":1148,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CD0F
// 首条位置: {"module":3205,"inlinee":118031}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CCB6
// 首条位置: {"module":3185,"inlinee":117942}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3185,"record_offset":7704,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/03185.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3185,"record_offset":7816,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/03185.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3185,"record_offset":7916,"parent_record_offset":7892,"ancestor_record_offset":7684,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03185.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3185,"record_offset":7840,"parent_record_offset":7816,"ancestor_record_offset":7684,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/03185.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3185,"record_offset":7948,"parent_record_offset":7864,"ancestor_record_offset":7684,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/03185.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3185,"record_offset":7864,"parent_record_offset":7840,"ancestor_record_offset":7684,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/03185.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3185,"record_offset":7892,"parent_record_offset":7864,"ancestor_record_offset":7684,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03185.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3185,"record_offset":7728,"parent_record_offset":7704,"ancestor_record_offset":7684,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/03185.jsonl.gz"}
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
// 完整结构及原始类型记录: merge_proposal_wall_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: merge_proposal_wall_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: merge_proposal_wall_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position
// 信息 ID: 6f5ed185fea75aa335f7901e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0x14d7460 -> 0x250a870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$0
// 信息 ID: fd9cdc0e7e7214a51e255874
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position; module=3205, inline_record=780; 不是独立调用边。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position; module=3205, inline_record=1000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$1
// 信息 ID: f958d530974ac83d2ea546e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position; module=3205, inline_record=1120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner
// 信息 ID: 2f3e71bebefed329ed54c2b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position @ 0x250a953 -> 0x250ad80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position @ 0x250ab65 -> 0x250ad80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$0
// 信息 ID: 88642504c7756aea1d75d7c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner; module=3205, inline_record=3596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$1
// 信息 ID: d0e539c4757cd311211fdeaa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1 <- system_decorator::collision_detection::intersect_wall_decorators<system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=3185, inline_record=7684; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=3185, inline_record=10280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
