// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/onboarding/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d9bd2166013beb13b339aa8ca781e77325442c609e4fc7c4d725d0672c17b862
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 316, 2658, 2678, 2683, 3942, 3960
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_onboarding; // namespace candidate
//   package identities: [{"name":"system-onboarding","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_display_system::hint_position_update
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2678,"record_offset":13888,"rva":34664256,"va":5403373376}
// 编译实例 RVAs: 0x210ef40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-27696877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8136159fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5403375560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-27698926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8125959fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5403377625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34653356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8543beffd","size":5,"soft_fail":false,"target_va":5368722192,"va":5403375543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34136230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e85a1ff7fd","size":5,"soft_fail":false,"target_va":5369240528,"va":5403376753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20050619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8450dcefe","size":5,"soft_fail":false,"target_va":5383325056,"va":5403375670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141619eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11492774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e85aa250ff","size":5,"soft_fail":false,"target_va":5391883952,"va":5403376721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287cab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7785146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8baca7600","size":5,"soft_fail":false,"target_va":5411162800,"va":5403377649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13681426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e812c3d000","size":5,"soft_fail":false,"target_va":5417057056,"va":5403375625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13242743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e87711ca00","size":5,"soft_fail":false,"target_va":5416620794,"va":5403378046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13243383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8f713ca00","size":5,"soft_fail":false,"target_va":5416621600,"va":5403378212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t13244133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8e516ca00","size":5,"soft_fail":false,"target_va":5416622208,"va":5403378070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13244117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8d516ca00","size":5,"soft_fail":false,"target_va":5416622240,"va":5403378118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_registry::ClutterRegistry::get_or_placeholder
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23963237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e89b5992fe","size":5,"soft_fail":false,"target_va":5379412656,"va":5403375888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22180719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8918cadfe","size":5,"soft_fail":false,"target_va":5381197120,"va":5403377834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22179127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8c992adfe","size":5,"soft_fail":false,"target_va":5381197520,"va":5403376642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::try_transform
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21721135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8d18fb4fe","size":5,"soft_fail":false,"target_va":5381654800,"va":5403375930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23872853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8abba93fe","size":5,"soft_fail":false,"target_va":5379502992,"va":5403375840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22142544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8b021aefe","size":5,"soft_fail":false,"target_va":5381234608,"va":5403377147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22471936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8001ba9fe","size":5,"soft_fail":false,"target_va":5380903536,"va":5403375467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22473980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e80413a9fe","size":5,"soft_fail":false,"target_va":5380902448,"va":5403376423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21617453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8d324b6fe","size":5,"soft_fail":false,"target_va":5381756080,"va":5403373528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23779275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8352895fe","size":5,"soft_fail":false,"target_va":5379597392,"va":5403376662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23779962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8862595fe","size":5,"soft_fail":false,"target_va":5379597264,"va":5403377221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::dashed_line
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23699125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e84b6196fe","size":5,"soft_fail":false,"target_va":5379677120,"va":5403376240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23699062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e88a6196fe","size":5,"soft_fail":false,"target_va":5379677168,"va":5403376225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23699048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8986196fe","size":5,"soft_fail":false,"target_va":5379677216,"va":5403376259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::world_space_to_screen_space
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24336705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8bfa68cfe","size":5,"soft_fail":false,"target_va":5379038368,"va":5403375068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::camera::CameraMatrices::eye_right
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-19962935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8c963cffe","size":5,"soft_fail":false,"target_va":5383412768,"va":5403375698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_points3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20067440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e890cbcdfe","size":5,"soft_fail":false,"target_va":5383307568,"va":5403375003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::points
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20052179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e82d07cefe","size":5,"soft_fail":false,"target_va":5383324096,"va":5403376270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::set_orientation_xz
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20052606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e88205cefe","size":5,"soft_fail":false,"target_va":5383323568,"va":5403376169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::translate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20051514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8c609cefe","size":5,"soft_fail":false,"target_va":5383324672,"va":5403376181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20095488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403373376,"raw_hex":"e8005ecdfe","size":5,"soft_fail":false,"target_va":5383278464,"va":5403373947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2678,"record_offset":21652,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2678,"record_offset":15156,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2678,"record_offset":15268,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2678,"record_offset":14108,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2678,"record_offset":24184,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2678,"record_offset":23896,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2678,"record_offset":23616,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2678,"record_offset":22072,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2678,"record_offset":19904,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2678,"record_offset":24216,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2678,"record_offset":25780,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2678,"record_offset":23532,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":2678,"record_offset":20780,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2678,"record_offset":21404,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2678,"record_offset":14564,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$44::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2678,"record_offset":14656,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":34262,"source_line":1478,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2678,"record_offset":19760,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2678,"record_offset":20600,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2678,"record_offset":20692,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_display_system::hint_position_update::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_display_system.rs
//     evidence={"module":2678,"record_offset":14752,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":108138,"source_line":192,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2678,"record_offset":20420,"parent_record_offset":20396,"ancestor_record_offset":13888,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2678,"record_offset":20444,"parent_record_offset":20420,"ancestor_record_offset":13888,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2678,"record_offset":25964,"parent_record_offset":25940,"ancestor_record_offset":13888,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2678,"record_offset":25988,"parent_record_offset":25964,"ancestor_record_offset":13888,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2678,"record_offset":20468,"parent_record_offset":20444,"ancestor_record_offset":13888,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2678,"record_offset":26016,"parent_record_offset":25988,"ancestor_record_offset":13888,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2678,"record_offset":20396,"parent_record_offset":20216,"ancestor_record_offset":13888,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2678,"record_offset":25940,"parent_record_offset":25852,"ancestor_record_offset":13888,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2678,"record_offset":14396,"parent_record_offset":14372,"ancestor_record_offset":13888,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2678,"record_offset":20156,"parent_record_offset":20128,"ancestor_record_offset":13888,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2678,"record_offset":25880,"parent_record_offset":25852,"ancestor_record_offset":13888,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2678,"record_offset":25852,"parent_record_offset":25828,"ancestor_record_offset":13888,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2678,"record_offset":14444,"parent_record_offset":14420,"ancestor_record_offset":13888,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2678,"record_offset":14420,"parent_record_offset":14396,"ancestor_record_offset":13888,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2678,"record_offset":20216,"parent_record_offset":20184,"ancestor_record_offset":13888,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2678,"record_offset":20184,"parent_record_offset":20156,"ancestor_record_offset":13888,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2678,"record_offset":25828,"parent_record_offset":25804,"ancestor_record_offset":13888,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2678,"record_offset":20252,"parent_record_offset":20216,"ancestor_record_offset":13888,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2678,"record_offset":21684,"parent_record_offset":21652,"ancestor_record_offset":13888,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2678,"record_offset":20080,"parent_record_offset":20008,"ancestor_record_offset":13888,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2678,"record_offset":14372,"parent_record_offset":14348,"ancestor_record_offset":13888,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2678,"record_offset":14348,"parent_record_offset":14316,"ancestor_record_offset":13888,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2678,"record_offset":15576,"parent_record_offset":15552,"ancestor_record_offset":13888,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2678,"record_offset":15600,"parent_record_offset":15576,"ancestor_record_offset":13888,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2678,"record_offset":20128,"parent_record_offset":20080,"ancestor_record_offset":13888,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2678,"record_offset":19972,"parent_record_offset":19940,"ancestor_record_offset":13888,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2678,"record_offset":23640,"parent_record_offset":23616,"ancestor_record_offset":13888,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2678,"record_offset":14316,"parent_record_offset":14284,"ancestor_record_offset":13888,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2678,"record_offset":18928,"parent_record_offset":15300,"ancestor_record_offset":13888,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2678,"record_offset":15784,"parent_record_offset":15300,"ancestor_record_offset":13888,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2678,"record_offset":17052,"parent_record_offset":15300,"ancestor_record_offset":13888,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2678,"record_offset":19180,"parent_record_offset":15300,"ancestor_record_offset":13888,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2678,"record_offset":22432,"parent_record_offset":22148,"ancestor_record_offset":13888,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2678,"record_offset":15700,"parent_record_offset":15300,"ancestor_record_offset":13888,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2678,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":13888,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2678,"record_offset":22196,"parent_record_offset":22148,"ancestor_record_offset":13888,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2678,"record_offset":22220,"parent_record_offset":22196,"ancestor_record_offset":13888,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2678,"record_offset":22336,"parent_record_offset":22304,"ancestor_record_offset":13888,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2678,"record_offset":22360,"parent_record_offset":22336,"ancestor_record_offset":13888,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2678,"record_offset":18468,"parent_record_offset":18444,"ancestor_record_offset":13888,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2678,"record_offset":17172,"parent_record_offset":17144,"ancestor_record_offset":13888,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2678,"record_offset":15900,"parent_record_offset":15868,"ancestor_record_offset":13888,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2678,"record_offset":18444,"parent_record_offset":18420,"ancestor_record_offset":13888,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2678,"record_offset":18492,"parent_record_offset":18468,"ancestor_record_offset":13888,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2678,"record_offset":17732,"parent_record_offset":17144,"ancestor_record_offset":13888,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2678,"record_offset":16428,"parent_record_offset":15868,"ancestor_record_offset":13888,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2678,"record_offset":18584,"parent_record_offset":18420,"ancestor_record_offset":13888,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2678,"record_offset":18632,"parent_record_offset":18608,"ancestor_record_offset":13888,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$92::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2678,"record_offset":18420,"parent_record_offset":15300,"ancestor_record_offset":13888,"inlinee":11825,"source_line":2520,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2678,"record_offset":14256,"parent_record_offset":14136,"ancestor_record_offset":13888,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2678,"record_offset":14284,"parent_record_offset":14256,"ancestor_record_offset":13888,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2678,"record_offset":15300,"parent_record_offset":15268,"ancestor_record_offset":13888,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2678,"record_offset":17144,"parent_record_offset":15300,"ancestor_record_offset":13888,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2678,"record_offset":15868,"parent_record_offset":15300,"ancestor_record_offset":13888,"inlinee":10708,"source_line":195,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2678,"record_offset":16380,"parent_record_offset":16356,"ancestor_record_offset":13888,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2678,"record_offset":16016,"parent_record_offset":15988,"ancestor_record_offset":13888,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2678,"record_offset":15988,"parent_record_offset":15960,"ancestor_record_offset":13888,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2678,"record_offset":16204,"parent_record_offset":16180,"ancestor_record_offset":13888,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2678,"record_offset":15960,"parent_record_offset":15928,"ancestor_record_offset":13888,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2678,"record_offset":15928,"parent_record_offset":15900,"ancestor_record_offset":13888,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2678,"record_offset":15552,"parent_record_offset":15300,"ancestor_record_offset":13888,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2678,"record_offset":19304,"parent_record_offset":19280,"ancestor_record_offset":13888,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2678,"record_offset":15468,"parent_record_offset":15300,"ancestor_record_offset":13888,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2678,"record_offset":16356,"parent_record_offset":16332,"ancestor_record_offset":13888,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2678,"record_offset":16332,"parent_record_offset":16308,"ancestor_record_offset":13888,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2678,"record_offset":14196,"parent_record_offset":14172,"ancestor_record_offset":13888,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2678,"record_offset":22124,"parent_record_offset":22100,"ancestor_record_offset":13888,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2678,"record_offset":22148,"parent_record_offset":22124,"ancestor_record_offset":13888,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2678,"record_offset":14220,"parent_record_offset":14196,"ancestor_record_offset":13888,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2678,"record_offset":14136,"parent_record_offset":14108,"ancestor_record_offset":13888,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2678,"record_offset":14172,"parent_record_offset":14136,"ancestor_record_offset":13888,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":2678,"record_offset":18516,"parent_record_offset":18492,"ancestor_record_offset":13888,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":2678,"record_offset":20276,"parent_record_offset":20252,"ancestor_record_offset":13888,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":2678,"record_offset":22468,"parent_record_offset":22432,"ancestor_record_offset":13888,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2678,"record_offset":24104,"parent_record_offset":24080,"ancestor_record_offset":13888,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2678,"record_offset":24128,"parent_record_offset":24104,"ancestor_record_offset":13888,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2678,"record_offset":21224,"parent_record_offset":20804,"ancestor_record_offset":13888,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2678,"record_offset":21132,"parent_record_offset":21076,"ancestor_record_offset":13888,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2678,"record_offset":20900,"parent_record_offset":20876,"ancestor_record_offset":13888,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2678,"record_offset":20932,"parent_record_offset":20848,"ancestor_record_offset":13888,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2678,"record_offset":20996,"parent_record_offset":20804,"ancestor_record_offset":13888,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2678,"record_offset":21548,"parent_record_offset":21524,"ancestor_record_offset":13888,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2678,"record_offset":20356,"parent_record_offset":20300,"ancestor_record_offset":13888,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":2678,"record_offset":23664,"parent_record_offset":23640,"ancestor_record_offset":13888,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2678,"record_offset":20300,"parent_record_offset":20276,"ancestor_record_offset":13888,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2678,"record_offset":20328,"parent_record_offset":20300,"ancestor_record_offset":13888,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2678,"record_offset":25908,"parent_record_offset":25880,"ancestor_record_offset":13888,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2678,"record_offset":14596,"parent_record_offset":14564,"ancestor_record_offset":13888,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2678,"record_offset":20628,"parent_record_offset":20600,"ancestor_record_offset":13888,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2678,"record_offset":19788,"parent_record_offset":19760,"ancestor_record_offset":13888,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2678,"record_offset":14688,"parent_record_offset":14656,"ancestor_record_offset":13888,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2678,"record_offset":14832,"parent_record_offset":14804,"ancestor_record_offset":13888,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2678,"record_offset":16108,"parent_record_offset":16044,"ancestor_record_offset":13888,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2678,"record_offset":24004,"parent_record_offset":23920,"ancestor_record_offset":13888,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":2678,"record_offset":24056,"parent_record_offset":24028,"ancestor_record_offset":13888,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":2678,"record_offset":24080,"parent_record_offset":24056,"ancestor_record_offset":13888,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2678,"record_offset":23000,"parent_record_offset":22976,"ancestor_record_offset":13888,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2678,"record_offset":16136,"parent_record_offset":16108,"ancestor_record_offset":13888,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2678,"record_offset":22304,"parent_record_offset":22220,"ancestor_record_offset":13888,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2678,"record_offset":16180,"parent_record_offset":15988,"ancestor_record_offset":13888,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2678,"record_offset":22404,"parent_record_offset":22148,"ancestor_record_offset":13888,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2678,"record_offset":20524,"parent_record_offset":20080,"ancestor_record_offset":13888,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2678,"record_offset":20548,"parent_record_offset":20524,"ancestor_record_offset":13888,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2678,"record_offset":25804,"parent_record_offset":25780,"ancestor_record_offset":13888,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2678,"record_offset":14512,"parent_record_offset":14488,"ancestor_record_offset":13888,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2678,"record_offset":15432,"parent_record_offset":15404,"ancestor_record_offset":13888,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2678,"record_offset":15492,"parent_record_offset":15468,"ancestor_record_offset":13888,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2678,"record_offset":19124,"parent_record_offset":15300,"ancestor_record_offset":13888,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2678,"record_offset":15516,"parent_record_offset":15492,"ancestor_record_offset":13888,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2678,"record_offset":16260,"parent_record_offset":16228,"ancestor_record_offset":13888,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2678,"record_offset":14488,"parent_record_offset":14316,"ancestor_record_offset":13888,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2678,"record_offset":23920,"parent_record_offset":23896,"ancestor_record_offset":13888,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2678,"record_offset":15404,"parent_record_offset":15300,"ancestor_record_offset":13888,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2678,"record_offset":22516,"parent_record_offset":22492,"ancestor_record_offset":13888,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2678,"record_offset":22540,"parent_record_offset":22516,"ancestor_record_offset":13888,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2678,"record_offset":22492,"parent_record_offset":22468,"ancestor_record_offset":13888,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":2678,"record_offset":20804,"parent_record_offset":20780,"ancestor_record_offset":13888,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":2678,"record_offset":21252,"parent_record_offset":20804,"ancestor_record_offset":13888,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2678,"record_offset":15036,"parent_record_offset":14752,"ancestor_record_offset":13888,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2678,"record_offset":21492,"parent_record_offset":21460,"ancestor_record_offset":13888,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2678,"record_offset":21460,"parent_record_offset":21428,"ancestor_record_offset":13888,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2678,"record_offset":21428,"parent_record_offset":21404,"ancestor_record_offset":13888,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2678,"record_offset":15060,"parent_record_offset":15036,"ancestor_record_offset":13888,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2678,"record_offset":14924,"parent_record_offset":14752,"ancestor_record_offset":13888,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2678,"record_offset":14868,"parent_record_offset":14752,"ancestor_record_offset":13888,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2678,"record_offset":14804,"parent_record_offset":14752,"ancestor_record_offset":13888,"inlinee":8901,"source_line":1579,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":2678,"record_offset":20876,"parent_record_offset":20848,"ancestor_record_offset":13888,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":2678,"record_offset":20848,"parent_record_offset":20804,"ancestor_record_offset":13888,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2678,"record_offset":19336,"parent_record_offset":19304,"ancestor_record_offset":13888,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2678,"record_offset":16228,"parent_record_offset":16204,"ancestor_record_offset":13888,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2678,"record_offset":21524,"parent_record_offset":21492,"ancestor_record_offset":13888,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_display_system::hint_position_update::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_display_system.rs
//     evidence={"module":2678,"record_offset":19940,"parent_record_offset":19904,"ancestor_record_offset":13888,"inlinee":108139,"source_line":297,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_display_system::hint_position_update::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_display_system.rs
//     evidence={"module":2678,"record_offset":22100,"parent_record_offset":22072,"ancestor_record_offset":13888,"inlinee":108140,"source_line":301,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_display_system::hint_position_update::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_display_system.rs
//     evidence={"module":2678,"record_offset":24028,"parent_record_offset":24004,"ancestor_record_offset":13888,"inlinee":67070,"source_line":323,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_display_system::hint_position_update::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_display_system.rs
//     evidence={"module":2678,"record_offset":24288,"parent_record_offset":24252,"ancestor_record_offset":13888,"inlinee":108141,"source_line":377,"archive":"_pdb/C/modules/02678.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::impl$0::add_onboarding
// 记录形式: compiled_procedure; instances=1
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":316,"record_offset":1524,"rva":7261328,"va":5375970448}
// 编译实例 RVAs: 0x6ecc90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..schedule..config..ScheduleConfigs$LT$T$GT$$u20$as$u20$bevy_ecs..schedule..config..IntoScheduleConfigs$LT$T$C$$LP$$RP$$GT$$GT$::run_if
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10133652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375970448,"raw_hex":"e894a09a00","size":5,"soft_fail":false,"target_va":5386104720,"va":5375971063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=6
//     target_aliases=118 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6771042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375970448,"raw_hex":"e89eae98ff","size":5,"soft_fail":false,"target_va":5369205664,"va":5375976701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140083b00
//     owner=unresolved_namespace; count=2
//     target_aliases=122 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6722497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375970448,"raw_hex":"e83f6c99ff","size":5,"soft_fail":false,"target_va":5369248512,"va":5375971004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5372946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375970448,"raw_hex":"e8ee03aeff","size":5,"soft_fail":false,"target_va":5370597904,"va":5375970845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108b830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10087293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375970448,"raw_hex":"e87deb9900","size":5,"soft_fail":false,"target_va":5386057776,"va":5375970478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108e510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10098761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375970448,"raw_hex":"e849189a00","size":5,"soft_fail":false,"target_va":5386069264,"va":5375970498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8823875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375970448,"raw_hex":"e843a48600","size":5,"soft_fail":false,"target_va":5384794864,"va":5375970984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10088706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375970448,"raw_hex":"e802f19900","size":5,"soft_fail":false,"target_va":5386059200,"va":5375970489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10136657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375970448,"raw_hex":"e851ac9a00","size":5,"soft_fail":false,"target_va":5386107488,"va":5375970826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10157814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375970448,"raw_hex":"e8f6fe9a00","size":5,"soft_fail":false,"target_va":5386129408,"va":5375971589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10142251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375970448,"raw_hex":"e82bc29a00","size":5,"soft_fail":false,"target_va":5386113344,"va":5375971088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10108060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375970448,"raw_hex":"e89c3c9a00","size":5,"soft_fail":false,"target_va":5386078576,"va":5375970511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":316,"record_offset":2196,"parent_record_offset":1524,"ancestor_record_offset":1524,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":316,"record_offset":1664,"parent_record_offset":1524,"ancestor_record_offset":1524,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::condition::common_conditions::not
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/condition.rs
//     evidence={"module":316,"record_offset":1764,"parent_record_offset":1524,"ancestor_record_offset":1524,"inlinee":20456,"source_line":1016,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":316,"record_offset":1692,"parent_record_offset":1524,"ancestor_record_offset":1524,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":316,"record_offset":1792,"parent_record_offset":1764,"ancestor_record_offset":1524,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":316,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":1524,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":316,"record_offset":3592,"parent_record_offset":3568,"ancestor_record_offset":1524,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":316,"record_offset":3640,"parent_record_offset":3616,"ancestor_record_offset":1524,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":316,"record_offset":3616,"parent_record_offset":3592,"ancestor_record_offset":1524,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":316,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":1524,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":316,"record_offset":3544,"parent_record_offset":3504,"ancestor_record_offset":1524,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":316,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":1524,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":316,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1524,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":316,"record_offset":2280,"parent_record_offset":2248,"ancestor_record_offset":1524,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":316,"record_offset":1732,"parent_record_offset":1692,"ancestor_record_offset":1524,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":316,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":1524,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":316,"record_offset":3788,"parent_record_offset":3764,"ancestor_record_offset":1524,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":316,"record_offset":3252,"parent_record_offset":3228,"ancestor_record_offset":1524,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":316,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":1524,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":316,"record_offset":3276,"parent_record_offset":3252,"ancestor_record_offset":1524,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":316,"record_offset":3480,"parent_record_offset":3456,"ancestor_record_offset":1524,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":316,"record_offset":2248,"parent_record_offset":2220,"ancestor_record_offset":1524,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":316,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":1524,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":316,"record_offset":2304,"parent_record_offset":2280,"ancestor_record_offset":1524,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":316,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":1524,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":316,"record_offset":2980,"parent_record_offset":2952,"ancestor_record_offset":1524,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":316,"record_offset":1876,"parent_record_offset":1764,"ancestor_record_offset":1524,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":316,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":1524,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":316,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":1524,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":316,"record_offset":2752,"parent_record_offset":2724,"ancestor_record_offset":1524,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":316,"record_offset":2724,"parent_record_offset":2696,"ancestor_record_offset":1524,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":316,"record_offset":3392,"parent_record_offset":3328,"ancestor_record_offset":1524,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":316,"record_offset":3204,"parent_record_offset":2248,"ancestor_record_offset":1524,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":316,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":1524,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":316,"record_offset":3740,"parent_record_offset":2248,"ancestor_record_offset":1524,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":316,"record_offset":3456,"parent_record_offset":3276,"ancestor_record_offset":1524,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":316,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":1524,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":316,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":1524,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":316,"record_offset":2868,"parent_record_offset":2844,"ancestor_record_offset":1524,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":316,"record_offset":3328,"parent_record_offset":3300,"ancestor_record_offset":1524,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":316,"record_offset":3364,"parent_record_offset":3328,"ancestor_record_offset":1524,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":316,"record_offset":3684,"parent_record_offset":3504,"ancestor_record_offset":1524,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":316,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":1524,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":316,"record_offset":2664,"parent_record_offset":2636,"ancestor_record_offset":1524,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":316,"record_offset":2696,"parent_record_offset":2664,"ancestor_record_offset":1524,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":316,"record_offset":2636,"parent_record_offset":2416,"ancestor_record_offset":1524,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":316,"record_offset":2496,"parent_record_offset":2416,"ancestor_record_offset":1524,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":316,"record_offset":3056,"parent_record_offset":2416,"ancestor_record_offset":1524,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":316,"record_offset":2580,"parent_record_offset":2416,"ancestor_record_offset":1524,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":316,"record_offset":2340,"parent_record_offset":2304,"ancestor_record_offset":1524,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":316,"record_offset":2904,"parent_record_offset":2796,"ancestor_record_offset":1524,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":316,"record_offset":2844,"parent_record_offset":2820,"ancestor_record_offset":1524,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":316,"record_offset":3148,"parent_record_offset":2416,"ancestor_record_offset":1524,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":316,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":1524,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":316,"record_offset":2392,"parent_record_offset":2368,"ancestor_record_offset":1524,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":316,"record_offset":2368,"parent_record_offset":2340,"ancestor_record_offset":1524,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":316,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":1524,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":316,"record_offset":2468,"parent_record_offset":2416,"ancestor_record_offset":1524,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":316,"record_offset":2796,"parent_record_offset":2416,"ancestor_record_offset":1524,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":316,"record_offset":3028,"parent_record_offset":2416,"ancestor_record_offset":1524,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00316.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::prompt_rollercoaster
// 记录形式: compiled_procedure; instances=1
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3960,"record_offset":272,"rva":42421136,"va":5411130256}
// 编译实例 RVAs: 0x2874b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14108a8c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25076765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411130256,"raw_hex":"e8e35b81fe","size":5,"soft_fail":false,"target_va":5386053824,"va":5411130584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ae730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23880943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411130256,"raw_hex":"e8119b93fe","size":5,"soft_fail":false,"target_va":5387249456,"va":5411130394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7939081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411130256,"raw_hex":"e8f7db86ff","size":5,"soft_fail":false,"target_va":5403191360,"va":5411130436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5251431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411130256,"raw_hex":"e867215000","size":5,"soft_fail":false,"target_va":5416381904,"va":5411130468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3960,"record_offset":1164,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3960,"record_offset":976,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":3960,"record_offset":412,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_content::ui_confirm_hint
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_content.rs
//     evidence={"module":3960,"record_offset":1220,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":134593,"source_line":108,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_state::HintDisplayState::add_popup_candidate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_state.rs
//     evidence={"module":3960,"record_offset":1336,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":108043,"source_line":150,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3960,"record_offset":1480,"parent_record_offset":1448,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3960,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3960,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3960,"record_offset":1448,"parent_record_offset":1396,"ancestor_record_offset":272,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3960,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3960,"record_offset":1364,"parent_record_offset":1336,"ancestor_record_offset":272,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3960,"record_offset":1396,"parent_record_offset":1364,"ancestor_record_offset":272,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3960,"record_offset":624,"parent_record_offset":524,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3960,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":272,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3960,"record_offset":524,"parent_record_offset":492,"ancestor_record_offset":272,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3960,"record_offset":1008,"parent_record_offset":976,"ancestor_record_offset":272,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":3960,"record_offset":492,"parent_record_offset":444,"ancestor_record_offset":272,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":3960,"record_offset":444,"parent_record_offset":412,"ancestor_record_offset":272,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3960,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3960,"record_offset":1040,"parent_record_offset":1008,"ancestor_record_offset":272,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3960,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":272,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3960,"record_offset":568,"parent_record_offset":524,"ancestor_record_offset":272,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3960,"record_offset":1068,"parent_record_offset":1040,"ancestor_record_offset":272,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3960,"record_offset":1128,"parent_record_offset":1008,"ancestor_record_offset":272,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3960,"record_offset":1596,"parent_record_offset":1396,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3960,"record_offset":1568,"parent_record_offset":1396,"ancestor_record_offset":272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3960,"record_offset":1304,"parent_record_offset":1220,"ancestor_record_offset":272,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3960,"record_offset":680,"parent_record_offset":524,"ancestor_record_offset":272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3960,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":272,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3960,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":272,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3960,"record_offset":1248,"parent_record_offset":1220,"ancestor_record_offset":272,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03960.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_wall_group_selection::WallGroupSelectionHintConditions>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>,bevy_ecs::message::message_writer::MessageWriter<system_onboarding::tutorial_completion::TutorialHintComplete>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>", "bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_wall_group_selection::WallGroupSelectionHintConditions>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>", "bevy_ecs::message::message_writer::MessageWriter<system_onboarding::tutorial_completion::TutorialHintComplete>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_wall_group_selection::WallGroupSelectionHintConditions>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::NotifyHintSystemGroupWasBookmarked>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_onboarding::tutorial_completion::TutorialHintComplete>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::toolbar::ToolbarUiIsInFocus>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<system_onboarding::hint_state::HintDisplayState>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::gui::TimeOfDayScreen>", "bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::toolbar::ToolbarUiIsInFocus>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_onboarding::hint_state::HintDisplayState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::TimeOfDayScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::toolbar::ToolbarUiIsInFocus>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<system_onboarding::hint_state::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_state::HintPlacement> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<system_onboarding::hint_state::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_state::HintPlacement> > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<system_onboarding::hint_state::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_state::HintPlacement> > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::DisplayRollercoasterPrompt>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::DisplayRollercoasterPrompt>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::DisplayRollercoasterPrompt>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_display_system::hint_position_update
// 信息 ID: 9636b33f2df64b4657de010e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ea836a0d48381605b99e3bd9: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<system_onboarding::hint_state::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_state::HintPlacement> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<system_onboarding::hint_state::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_state::HintPlacement> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<system_onboarding::hint_state::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_state::HintPlacement> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x1093550 -> 0x210ef40 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<system_onboarding::hint_state::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_state::HintPlacement> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<system_onboarding::hint_state::HintLayoutMemo>,ref_mut$<system_onboarding::hint_display_system::HintSpring>,ref$<enum2$<system_onboarding::hint_state::HintPlacement> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x20f20ec -> 0x210ef40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::impl$0::add_onboarding
// 信息 ID: d626a2f6681b5b6fd5a07263
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x4213fe -> 0x6ecc90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::prompt_rollercoaster
// 信息 ID: 05f403111ee4c6424449b1f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e5740d704abdb946d6878489: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::DisplayRollercoasterPrompt>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::DisplayRollercoasterPrompt>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::DisplayRollercoasterPrompt>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)> > @ 0x20f0269 -> 0x2874b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::DisplayRollercoasterPrompt>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::DisplayRollercoasterPrompt>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)> @ 0x210bc46 -> 0x2874b90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
