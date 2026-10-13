// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/clutter/src/clutter/add_lantern_water_setdresssing.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 231aa48a9b730dfc4587c4e4a41640305fee3bf2a5ed85a8df5a49004504283d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: add_lantern_water_setdresssing.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3513
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::add_lantern_water_setdresssing::add_lantern_water_setdresssing
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3513,"record_offset":252,"rva":28127520,"va":5396836640}
// 编译实例 RVAs: 0x1ad3120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: add_lantern_water_setdresssing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23686932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e8ec9096fe","size":5,"soft_fail":false,"target_va":5373149840,"va":5396836767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14054a000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: add_lantern_water_setdresssing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22583418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e88667a7fe","size":5,"soft_fail":false,"target_va":5374255104,"va":5396838517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: add_lantern_water_setdresssing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19894465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e83f6fd0fe","size":5,"soft_fail":false,"target_va":5376943968,"va":5396838428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141adf890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: add_lantern_water_setdresssing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t49190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e826c00000","size":5,"soft_fail":false,"target_va":5396887696,"va":5396838501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: add_lantern_water_setdresssing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6756439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e857186700","size":5,"soft_fail":false,"target_va":5403594144,"va":5396837700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25530039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e849717afe","size":5,"soft_fail":false,"target_va":5371308800,"va":5396838834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6140034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e882b05d00","size":5,"soft_fail":false,"target_va":5402979152,"va":5396839113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6139696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e830af5d00","size":5,"soft_fail":false,"target_va":5402978944,"va":5396839243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::GladeType::is_winter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22581690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e8466ea7fe","size":5,"soft_fail":false,"target_va":5374255120,"va":5396836805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22474829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e8b30fa9fe","size":5,"soft_fail":false,"target_va":5374362576,"va":5396837400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter_terrain_decorators
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22118575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e8517faefe","size":5,"soft_fail":false,"target_va":5374718368,"va":5396836938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-10987643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e8855758ff","size":5,"soft_fail":false,"target_va":5385850032,"va":5396837670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18869859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e89d11e0fe","size":5,"soft_fail":false,"target_va":5377967920,"va":5396837774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18869588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e8ac12e0fe","size":5,"soft_fail":false,"target_va":5377967856,"va":5396837439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-19898337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e81f60d0fe","size":5,"soft_fail":false,"target_va":5376939952,"va":5396838284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14581804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396836640,"raw_hex":"e8d47f21ff","size":5,"soft_fail":false,"target_va":5382255664,"va":5396837463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3513,"record_offset":3008,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3513,"record_offset":4880,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3513,"record_offset":3360,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":3513,"record_offset":532,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5070,"source_line":65,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3513,"record_offset":3148,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_autumn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3513,"record_offset":4852,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":22564,"source_line":210,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3513,"record_offset":504,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":12658,"source_line":205,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3513,"record_offset":476,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":21417,"source_line":126,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":4640,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":3700,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14662,"source_line":1012,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":3644,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3513,"record_offset":4936,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3513,"record_offset":3232,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":3513,"record_offset":3756,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3513,"record_offset":3084,"parent_record_offset":3052,"ancestor_record_offset":252,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3513,"record_offset":3052,"parent_record_offset":3008,"ancestor_record_offset":252,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3513,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":252,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3513,"record_offset":3512,"parent_record_offset":3488,"ancestor_record_offset":252,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3513,"record_offset":3536,"parent_record_offset":3512,"ancestor_record_offset":252,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3513,"record_offset":3196,"parent_record_offset":3172,"ancestor_record_offset":252,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3513,"record_offset":4160,"parent_record_offset":3756,"ancestor_record_offset":252,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3513,"record_offset":1016,"parent_record_offset":984,"ancestor_record_offset":252,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3513,"record_offset":1080,"parent_record_offset":1044,"ancestor_record_offset":252,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3513,"record_offset":3268,"parent_record_offset":3232,"ancestor_record_offset":252,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3513,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":252,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3513,"record_offset":4032,"parent_record_offset":3976,"ancestor_record_offset":252,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3513,"record_offset":3604,"parent_record_offset":3424,"ancestor_record_offset":252,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":3513,"record_offset":620,"parent_record_offset":564,"ancestor_record_offset":252,"inlinee":5074,"source_line":839,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$35::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":3513,"record_offset":564,"parent_record_offset":532,"ancestor_record_offset":252,"inlinee":5072,"source_line":605,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":3513,"record_offset":1680,"parent_record_offset":1652,"ancestor_record_offset":252,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":3513,"record_offset":1652,"parent_record_offset":564,"ancestor_record_offset":252,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3513,"record_offset":716,"parent_record_offset":688,"ancestor_record_offset":252,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3513,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":252,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3513,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":252,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3513,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":252,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3513,"record_offset":3668,"parent_record_offset":3644,"ancestor_record_offset":252,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3513,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":252,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3513,"record_offset":3864,"parent_record_offset":3840,"ancestor_record_offset":252,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3513,"record_offset":3724,"parent_record_offset":3700,"ancestor_record_offset":252,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3513,"record_offset":4576,"parent_record_offset":4548,"ancestor_record_offset":252,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3513,"record_offset":688,"parent_record_offset":620,"ancestor_record_offset":252,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3513,"record_offset":1936,"parent_record_offset":1904,"ancestor_record_offset":252,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3513,"record_offset":3488,"parent_record_offset":3464,"ancestor_record_offset":252,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3513,"record_offset":3464,"parent_record_offset":3424,"ancestor_record_offset":252,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3513,"record_offset":660,"parent_record_offset":620,"ancestor_record_offset":252,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3513,"record_offset":1904,"parent_record_offset":1736,"ancestor_record_offset":252,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3513,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":252,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$18::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3513,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":252,"inlinee":8605,"source_line":2242,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$23::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3513,"record_offset":1764,"parent_record_offset":1736,"ancestor_record_offset":252,"inlinee":21682,"source_line":2282,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3513,"record_offset":1200,"parent_record_offset":840,"ancestor_record_offset":252,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3513,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":252,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3513,"record_offset":1140,"parent_record_offset":1116,"ancestor_record_offset":252,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3513,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":252,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$4::iter_terrain_decorators::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":3513,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":252,"inlinee":21672,"source_line":155,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3513,"record_offset":4280,"parent_record_offset":4244,"ancestor_record_offset":252,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":4124,"parent_record_offset":3756,"ancestor_record_offset":252,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":3952,"parent_record_offset":3756,"ancestor_record_offset":252,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":4740,"parent_record_offset":4704,"ancestor_record_offset":252,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":4704,"parent_record_offset":4664,"ancestor_record_offset":252,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":4664,"parent_record_offset":4640,"ancestor_record_offset":252,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":3976,"parent_record_offset":3952,"ancestor_record_offset":252,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":3896,"parent_record_offset":3756,"ancestor_record_offset":252,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":4548,"parent_record_offset":3756,"ancestor_record_offset":252,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":3840,"parent_record_offset":3756,"ancestor_record_offset":252,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3513,"record_offset":4068,"parent_record_offset":3756,"ancestor_record_offset":252,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":3513,"record_offset":4216,"parent_record_offset":3756,"ancestor_record_offset":252,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":3513,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":252,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3513,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":252,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3513,"record_offset":772,"parent_record_offset":744,"ancestor_record_offset":252,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3513,"record_offset":1116,"parent_record_offset":840,"ancestor_record_offset":252,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3513,"record_offset":2144,"parent_record_offset":2056,"ancestor_record_offset":252,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3513,"record_offset":840,"parent_record_offset":800,"ancestor_record_offset":252,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3513,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":252,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3513,"record_offset":2116,"parent_record_offset":2056,"ancestor_record_offset":252,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3513,"record_offset":2088,"parent_record_offset":2056,"ancestor_record_offset":252,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3513,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":252,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3513,"record_offset":920,"parent_record_offset":888,"ancestor_record_offset":252,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3513,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":252,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3513,"record_offset":952,"parent_record_offset":888,"ancestor_record_offset":252,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3513,"record_offset":888,"parent_record_offset":840,"ancestor_record_offset":252,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3513,"record_offset":800,"parent_record_offset":772,"ancestor_record_offset":252,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3513,"record_offset":984,"parent_record_offset":840,"ancestor_record_offset":252,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3513,"record_offset":1044,"parent_record_offset":984,"ancestor_record_offset":252,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3513,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":252,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3513,"record_offset":744,"parent_record_offset":716,"ancestor_record_offset":252,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3513,"record_offset":4776,"parent_record_offset":4740,"ancestor_record_offset":252,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3513,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":252,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3513,"record_offset":3424,"parent_record_offset":3392,"ancestor_record_offset":252,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3513,"record_offset":3296,"parent_record_offset":3232,"ancestor_record_offset":252,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3513,"record_offset":3392,"parent_record_offset":3360,"ancestor_record_offset":252,"inlinee":26825,"source_line":115,"archive":"_pdb/A/modules/03513.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":3513,"record_offset":4244,"parent_record_offset":3756,"ancestor_record_offset":252,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/03513.jsonl.gz"}
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
// 完整结构及原始类型记录: add_lantern_water_setdresssing.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: add_lantern_water_setdresssing.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: add_lantern_water_setdresssing.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>", "bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::add_lantern_water_setdresssing::add_lantern_water_setdresssing
// 信息 ID: 6145bc54aafca20094701d88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ce731adb5a49c0179470b31d: void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0x14c5f9e -> 0x1ad3120 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> @ 0x14e95dc -> 0x1ad3120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
