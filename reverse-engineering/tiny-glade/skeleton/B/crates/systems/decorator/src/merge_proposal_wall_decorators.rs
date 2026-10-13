// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/merge_proposal_wall_decorators.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: bffb6a4b775ed649960bd36f81d3b46e0abf227d79fb2a758f1b20190078f93f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: merge_proposal_wall_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3051, 3064, 4078
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
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
// 首条位置: {"module":3064,"record_offset":1256,"rva":33561920,"va":5402271040}
// 编译实例 RVAs: 0x2001d40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002370
//     owner=unresolved_namespace; count=2
//     target_aliases=1937 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33554066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e86e0100fe","size":5,"soft_fail":false,"target_va":5368718192,"va":5402272253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27594550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e8caf05afe","size":5,"soft_fail":false,"target_va":5374676688,"va":5402271233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27191150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e8921861fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5402272649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24964234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e8761383fe","size":5,"soft_fail":false,"target_va":5377307216,"va":5402271445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21873239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e8a93db2fe","size":5,"soft_fail":false,"target_va":5380398224,"va":5402271458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20382519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e8c9fcc8fe","size":5,"soft_fail":false,"target_va":5381889520,"va":5402272034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20382611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e86dfcc8fe","size":5,"soft_fail":false,"target_va":5381889600,"va":5402272206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20382252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e8d4fdc8fe","size":5,"soft_fail":false,"target_va":5381889888,"va":5402272135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14307180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e894b025ff","size":5,"soft_fail":false,"target_va":5387964304,"va":5402271479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fe5900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: merge_proposal_wall_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-117161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e85736feff","size":5,"soft_fail":false,"target_va":5402155264,"va":5402272420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9270007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e8f7728d00","size":5,"soft_fail":false,"target_va":5411541808,"va":5402271796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9269307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e83b708d00","size":5,"soft_fail":false,"target_va":5411542064,"va":5402272752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9269171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e8b36f8d00","size":5,"soft_fail":false,"target_va":5411541904,"va":5402272728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24964495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e8711283fe","size":5,"soft_fail":false,"target_va":5377307552,"va":5402272042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::max_rank
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24964723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e88d1183fe","size":5,"soft_fail":false,"target_va":5377306928,"va":5402271646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21869908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e8ac4ab2fe","size":5,"soft_fail":false,"target_va":5380401520,"va":5402271423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21875341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402271040,"raw_hex":"e87335b2fe","size":5,"soft_fail":false,"target_va":5380396720,"va":5402272056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3064,"record_offset":3824,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3064,"record_offset":3796,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3064,"record_offset":3468,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$40::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3064,"record_offset":3088,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":6199,"source_line":2499,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3064,"record_offset":3852,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3064,"record_offset":2568,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3064,"record_offset":3528,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3064,"record_offset":3612,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3064,"record_offset":3740,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3064,"record_offset":1492,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::impl$0::snapping_direction
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":3064,"record_offset":3116,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":25468,"source_line":63,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::try_merge_decorators
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":3064,"record_offset":2660,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":106229,"source_line":77,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$0::classify_side
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3064,"record_offset":3584,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":106264,"source_line":43,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3064,"record_offset":3704,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":25625,"source_line":213,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3064,"record_offset":3320,"parent_record_offset":1256,"ancestor_record_offset":1256,"inlinee":25152,"source_line":70,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3064,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":1256,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3064,"record_offset":4036,"parent_record_offset":3936,"ancestor_record_offset":1256,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3064,"record_offset":3968,"parent_record_offset":3936,"ancestor_record_offset":1256,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3064,"record_offset":3936,"parent_record_offset":3908,"ancestor_record_offset":1256,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3064,"record_offset":3908,"parent_record_offset":3880,"ancestor_record_offset":1256,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3064,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":1256,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3064,"record_offset":2320,"parent_record_offset":2296,"ancestor_record_offset":1256,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3064,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":1256,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3064,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":1256,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3064,"record_offset":3400,"parent_record_offset":3376,"ancestor_record_offset":1256,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3064,"record_offset":4204,"parent_record_offset":4180,"ancestor_record_offset":1256,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3064,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":1256,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3064,"record_offset":2388,"parent_record_offset":1692,"ancestor_record_offset":1256,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3064,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":1256,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3064,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":1256,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":3064,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":1256,"inlinee":15378,"source_line":104,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":3064,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":1256,"inlinee":9239,"source_line":49,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":3064,"record_offset":3036,"parent_record_offset":2984,"ancestor_record_offset":1256,"inlinee":9245,"source_line":53,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3064,"record_offset":2692,"parent_record_offset":2660,"ancestor_record_offset":1256,"inlinee":12536,"source_line":2758,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3064,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":1256,"inlinee":12540,"source_line":2765,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3064,"record_offset":2888,"parent_record_offset":2660,"ancestor_record_offset":1256,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3064,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":1256,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3064,"record_offset":4004,"parent_record_offset":3968,"ancestor_record_offset":1256,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3064,"record_offset":2060,"parent_record_offset":2032,"ancestor_record_offset":1256,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3064,"record_offset":2032,"parent_record_offset":2004,"ancestor_record_offset":1256,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3064,"record_offset":3644,"parent_record_offset":3612,"ancestor_record_offset":1256,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3064,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":1256,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3064,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":1256,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3064,"record_offset":3880,"parent_record_offset":3852,"ancestor_record_offset":1256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3064,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":1256,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3064,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":1256,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3064,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":1256,"inlinee":24851,"source_line":21,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3064,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":1256,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3064,"record_offset":3228,"parent_record_offset":3176,"ancestor_record_offset":1256,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3064,"record_offset":2600,"parent_record_offset":2568,"ancestor_record_offset":1256,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3064,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":1256,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3064,"record_offset":1972,"parent_record_offset":1944,"ancestor_record_offset":1256,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3064,"record_offset":2004,"parent_record_offset":1972,"ancestor_record_offset":1256,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3064,"record_offset":1944,"parent_record_offset":1692,"ancestor_record_offset":1256,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3064,"record_offset":1804,"parent_record_offset":1692,"ancestor_record_offset":1256,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3064,"record_offset":2416,"parent_record_offset":1692,"ancestor_record_offset":1256,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3064,"record_offset":1888,"parent_record_offset":1692,"ancestor_record_offset":1256,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":3064,"record_offset":1776,"parent_record_offset":1692,"ancestor_record_offset":1256,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":3064,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":1256,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3064,"record_offset":1560,"parent_record_offset":1524,"ancestor_record_offset":1256,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3064,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":1256,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3064,"record_offset":2272,"parent_record_offset":2104,"ancestor_record_offset":1256,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3064,"record_offset":1616,"parent_record_offset":1584,"ancestor_record_offset":1256,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3064,"record_offset":2212,"parent_record_offset":2128,"ancestor_record_offset":1256,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3064,"record_offset":2156,"parent_record_offset":2128,"ancestor_record_offset":1256,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3064,"record_offset":2508,"parent_record_offset":1692,"ancestor_record_offset":1256,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3064,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":1256,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3064,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":1256,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3064,"record_offset":1644,"parent_record_offset":1584,"ancestor_record_offset":1256,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3064,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":1256,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3064,"record_offset":1748,"parent_record_offset":1692,"ancestor_record_offset":1256,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3064,"record_offset":2104,"parent_record_offset":1692,"ancestor_record_offset":1256,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3064,"record_offset":1524,"parent_record_offset":1492,"ancestor_record_offset":1256,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_wall_decorators.rs
//     evidence={"module":3064,"record_offset":3496,"parent_record_offset":3468,"ancestor_record_offset":1256,"inlinee":106263,"source_line":110,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":3064,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":1256,"inlinee":106230,"source_line":84,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":3064,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":1256,"inlinee":25228,"source_line":89,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3064,"record_offset":3352,"parent_record_offset":3320,"ancestor_record_offset":1256,"inlinee":25154,"source_line":39,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3064,"record_offset":3376,"parent_record_offset":3352,"ancestor_record_offset":1256,"inlinee":25156,"source_line":58,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3064,"record_offset":3280,"parent_record_offset":3228,"ancestor_record_offset":1256,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3064,"record_offset":3252,"parent_record_offset":3228,"ancestor_record_offset":1256,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/03064.jsonl.gz"}
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
// 首条位置: {"module":3064,"record_offset":264,"rva":33560704,"va":5402269824}
// 编译实例 RVAs: 0x2001880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9270752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402269824,"raw_hex":"e8e0758d00","size":5,"soft_fail":false,"target_va":5411541776,"va":5402271019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402269824,"raw_hex":"e8db030000","size":5,"soft_fail":false,"target_va":5402271040,"va":5402270048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3064,"record_offset":440,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6059,"source_line":3258,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3064,"record_offset":1060,"parent_record_offset":1032,"ancestor_record_offset":264,"inlinee":14050,"source_line":1502,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3064,"record_offset":964,"parent_record_offset":880,"ancestor_record_offset":264,"inlinee":18118,"source_line":39,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3064,"record_offset":852,"parent_record_offset":472,"ancestor_record_offset":264,"inlinee":18108,"source_line":142,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3064,"record_offset":516,"parent_record_offset":472,"ancestor_record_offset":264,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3064,"record_offset":1032,"parent_record_offset":964,"ancestor_record_offset":264,"inlinee":14049,"source_line":3265,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3064,"record_offset":472,"parent_record_offset":440,"ancestor_record_offset":264,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3064,"record_offset":1208,"parent_record_offset":880,"ancestor_record_offset":264,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3064,"record_offset":760,"parent_record_offset":548,"ancestor_record_offset":264,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3064,"record_offset":1096,"parent_record_offset":1060,"ancestor_record_offset":264,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3064,"record_offset":1152,"parent_record_offset":1124,"ancestor_record_offset":264,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3064,"record_offset":824,"parent_record_offset":472,"ancestor_record_offset":264,"inlinee":6199,"source_line":2499,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3064,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":264,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3064,"record_offset":640,"parent_record_offset":588,"ancestor_record_offset":264,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3064,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":264,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3064,"record_offset":548,"parent_record_offset":516,"ancestor_record_offset":264,"inlinee":11874,"source_line":313,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3064,"record_offset":880,"parent_record_offset":852,"ancestor_record_offset":264,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3064,"record_offset":588,"parent_record_offset":548,"ancestor_record_offset":264,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3064,"record_offset":672,"parent_record_offset":588,"ancestor_record_offset":264,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_wall_decorators.rs
//     evidence={"module":3064,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":264,"inlinee":106261,"source_line":51,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_wall_decorators.rs
//     evidence={"module":3064,"record_offset":1124,"parent_record_offset":1096,"ancestor_record_offset":264,"inlinee":106262,"source_line":65,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F15
// 首条位置: {"module":3064,"inlinee":106261}
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
// IPI IDs: 0x19F16
// 首条位置: {"module":3064,"inlinee":106262}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3064,"record_offset":1152,"parent_record_offset":1124,"ancestor_record_offset":1124,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03064.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F17
// 首条位置: {"module":3064,"inlinee":106263}
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
// IPI IDs: 0x19EE5
// 首条位置: {"module":3051,"inlinee":106213}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3051,"record_offset":872,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3051,"record_offset":984,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3051,"record_offset":1084,"parent_record_offset":1056,"ancestor_record_offset":852,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3051,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":852,"inlinee":12925,"source_line":236,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3051,"record_offset":1116,"parent_record_offset":1032,"ancestor_record_offset":852,"inlinee":12927,"source_line":237,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3051,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":852,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3051,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":852,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3051,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":852,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/03051.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position
// 信息 ID: d29ae5fedb576b0c9e1a583a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0x127d312 -> 0x2001880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$0
// 信息 ID: a74cdc653bf9f4818c62cb31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position; module=3064, inline_record=784; 不是独立调用边。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position; module=3064, inline_record=1004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position::closure$1
// 信息 ID: 8120b5ea6717f5e1146819ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position; module=3064, inline_record=1124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner
// 信息 ID: 4f7efbed176a8e898595218c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position @ 0x2001960 -> 0x2001d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position @ 0x2001b4f -> 0x2001d40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$0
// 信息 ID: ddbd3d0c51901f527b8e7fce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner; module=3064, inline_record=3496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$1
// 信息 ID: e9391e98c75962aab4aec96d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1 <- system_decorator::collision_detection::intersect_wall_decorators<system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=3051, inline_record=852; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=3051, inline_record=2256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
