// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/color/src/ui_color_decorator.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 663ab73a220238e73d0511ff191d6c47e615e536ccaab029322956176dfe5a2c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_color_decorator.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2235, 3479, 3494
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
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3479,"record_offset":1080,"rva":28054016,"va":5396763136}
// 编译实例 RVAs: 0x1ac1200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24014905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e8c78f91fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5396764612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23613579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e875af97fe","size":5,"soft_fail":false,"target_va":5373149840,"va":5396763414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bacb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22046264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e8c899affe","size":5,"soft_fail":false,"target_va":5374717104,"va":5396763363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21550691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e89d29b7fe","size":5,"soft_fail":false,"target_va":5375213232,"va":5396763918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406edb80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20790164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e86cc4c2fe","size":5,"soft_fail":false,"target_va":5375974272,"va":5396764431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac5330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e87d3e0000","size":5,"soft_fail":false,"target_va":5396779824,"va":5396763822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac6100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e8334c0000","size":5,"soft_fail":false,"target_va":5396783360,"va":5396763848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6829702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e886366800","size":5,"soft_fail":false,"target_va":5403593360,"va":5396763653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19233637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e89b84dafe","size":5,"soft_fail":false,"target_va":5377529888,"va":5396763520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventWriter$LT$E$GT$::send
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18836027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e8c595e0fe","size":5,"soft_fail":false,"target_va":5377928128,"va":5396764150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6214793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e889d45e00","size":5,"soft_fail":false,"target_va":5402979152,"va":5396764354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22401401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e8872eaafe","size":5,"soft_fail":false,"target_va":5374362576,"va":5396763972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_outline::DecoratorOutline::display
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21545518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e8d23db7fe","size":5,"soft_fail":false,"target_va":5375218592,"va":5396764105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22047343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e89195affe","size":5,"soft_fail":false,"target_va":5374716704,"va":5396764042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_outline
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21552310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e84a23b7fe","size":5,"soft_fail":false,"target_va":5375211696,"va":5396764001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21551258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e86627b7fe","size":5,"soft_fail":false,"target_va":5375212688,"va":5396763941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21329089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e83f8bbafe","size":5,"soft_fail":false,"target_va":5375434976,"va":5396764060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_raycast_w_params
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22254102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e8ea6dacfe","size":5,"soft_fail":false,"target_va":5374509168,"va":5396763265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21213841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e86f4dbcfe","size":5,"soft_fail":false,"target_va":5375550288,"va":5396764124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21213441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e8ff4ebcfe","size":5,"soft_fail":false,"target_va":5375550448,"va":5396763884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20992125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e883afbffe","size":5,"soft_fail":false,"target_va":5375771552,"va":5396763672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_click
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20992088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e8a8afbffe","size":5,"soft_fail":false,"target_va":5375771664,"va":5396763747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20992112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e890afbffe","size":5,"soft_fail":false,"target_va":5375771600,"va":5396763707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20992461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e833aebffe","size":5,"soft_fail":false,"target_va":5375771168,"va":5396763624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20992499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e80daebffe","size":5,"soft_fail":false,"target_va":5375771280,"va":5396763774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::components::transform::Transform::compute_affine
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396763136,"raw_hex":"e85bfbffff","size":5,"soft_fail":false,"target_va":5396762800,"va":5396763984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3479,"record_offset":1308,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3479,"record_offset":1248,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3479,"record_offset":1760,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3479,"record_offset":1848,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3479,"record_offset":1704,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::with_flags
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":3479,"record_offset":1616,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":90032,"source_line":112,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3479,"record_offset":1392,"parent_record_offset":1080,"ancestor_record_offset":1080,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3479,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":1080,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3479,"record_offset":1984,"parent_record_offset":1928,"ancestor_record_offset":1080,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3479,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":1080,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3479,"record_offset":1956,"parent_record_offset":1928,"ancestor_record_offset":1080,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3479,"record_offset":1928,"parent_record_offset":1900,"ancestor_record_offset":1080,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3479,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":1080,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3479,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":1080,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3479,"record_offset":1460,"parent_record_offset":1424,"ancestor_record_offset":1080,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3479,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":1080,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3479,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":1080,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3479,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":1080,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::WidgetFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":3479,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":1080,"inlinee":90035,"source_line":609,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3479,"record_offset":1544,"parent_record_offset":1424,"ancestor_record_offset":1080,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3479,"record_offset":1424,"parent_record_offset":1392,"ancestor_record_offset":1080,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3479,"record_offset":1576,"parent_record_offset":1544,"ancestor_record_offset":1080,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":3479,"record_offset":1644,"parent_record_offset":1616,"ancestor_record_offset":1080,"inlinee":106705,"source_line":77,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":3479,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":1080,"inlinee":106706,"source_line":121,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A0D1
// 首条位置: {"module":3479,"inlinee":106705}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":3479,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":90035,"source_line":609,"archive":"_pdb/A/modules/03479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2235,"record_offset":40804,"rva":21676112,"va":5390385232}
// 编译实例 RVAs: 0x14ac050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17635593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390385232,"raw_hex":"e8f7e6f2fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5390385284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17635719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390385232,"raw_hex":"e879e6f2fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5390385426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12592753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390385232,"raw_hex":"e87126c000","size":5,"soft_fail":false,"target_va":5402978144,"va":5390385386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall::color_cursor_icon
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390385232,"raw_hex":"e898e2ffff","size":5,"soft_fail":false,"target_va":5390377808,"va":5390385331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2235,"record_offset":41200,"parent_record_offset":40804,"ancestor_record_offset":40804,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2235,"record_offset":41316,"parent_record_offset":41292,"ancestor_record_offset":40804,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2235,"record_offset":41340,"parent_record_offset":41316,"ancestor_record_offset":40804,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2235,"record_offset":41564,"parent_record_offset":41540,"ancestor_record_offset":40804,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2235,"record_offset":41264,"parent_record_offset":41224,"ancestor_record_offset":40804,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2235,"record_offset":41292,"parent_record_offset":41264,"ancestor_record_offset":40804,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2235,"record_offset":41540,"parent_record_offset":41516,"ancestor_record_offset":40804,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2235,"record_offset":41516,"parent_record_offset":41492,"ancestor_record_offset":40804,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2235,"record_offset":41432,"parent_record_offset":41408,"ancestor_record_offset":40804,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2235,"record_offset":41408,"parent_record_offset":41224,"ancestor_record_offset":40804,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2235,"record_offset":41456,"parent_record_offset":41432,"ancestor_record_offset":40804,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2235,"record_offset":41492,"parent_record_offset":41224,"ancestor_record_offset":40804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2235,"record_offset":41364,"parent_record_offset":41340,"ancestor_record_offset":40804,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_decorator::ui_color_windows_and_flags::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_decorator.rs
//     evidence={"module":2235,"record_offset":41224,"parent_record_offset":41200,"ancestor_record_offset":40804,"inlinee":81113,"source_line":88,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$1
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13CD9
// 首条位置: {"module":2235,"inlinee":81113}
// 编译实例 RVAs: 0x14ac050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17635593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390385232,"raw_hex":"e8f7e6f2fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5390385284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17635719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390385232,"raw_hex":"e879e6f2fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5390385426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12592753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390385232,"raw_hex":"e87126c000","size":5,"soft_fail":false,"target_va":5402978144,"va":5390385386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall::color_cursor_icon
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390385232,"raw_hex":"e898e2ffff","size":5,"soft_fail":false,"target_va":5390377808,"va":5390385331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2235,"record_offset":41264,"parent_record_offset":41224,"ancestor_record_offset":41224,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2235,"record_offset":41408,"parent_record_offset":41224,"ancestor_record_offset":41224,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2235,"record_offset":41492,"parent_record_offset":41224,"ancestor_record_offset":41224,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2235,"record_offset":41316,"parent_record_offset":41292,"ancestor_record_offset":41224,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2235,"record_offset":41340,"parent_record_offset":41316,"ancestor_record_offset":41224,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2235,"record_offset":41564,"parent_record_offset":41540,"ancestor_record_offset":41224,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2235,"record_offset":41292,"parent_record_offset":41264,"ancestor_record_offset":41224,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2235,"record_offset":41540,"parent_record_offset":41516,"ancestor_record_offset":41224,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2235,"record_offset":41516,"parent_record_offset":41492,"ancestor_record_offset":41224,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2235,"record_offset":41432,"parent_record_offset":41408,"ancestor_record_offset":41224,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2235,"record_offset":41456,"parent_record_offset":41432,"ancestor_record_offset":41224,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2235,"record_offset":41364,"parent_record_offset":41340,"ancestor_record_offset":41224,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A0D2
// 首条位置: {"module":3479,"inlinee":106706}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3479,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":1788,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/03479.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_color_decorator.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_color_decorator.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_color_decorator.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >
// 信息 ID: 86e5bd0cb835e40baaa710e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [系统接口类型拼写] deferred_command_interface: bevy_ecs::system::commands::Commands
// [类型关系] ref$ -> ["country_core::asset_libraries::mesh_library::AssetMeshLibrary"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["country_core::asset_libraries::shader_library::AssetShaderLibrary"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["bevy_ecs::system::commands::Commands"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "ref_mut$<bevy_ecs::system::commands::Commands>"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnOnce::call_once -> ["system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$1", "tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags
// 信息 ID: 8886bf61b0c1372ef8f886cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d6588cd8e44231543cb9bdea: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)> > @ 0x149f849 -> 0x1ac1200 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>)> @ 0x14abc2b -> 0x1ac1200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$0
// 信息 ID: 95e48d564c858e2a0fa72474
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_decorator::ui_color_windows_and_flags; module=3479, inline_record=1644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$1
// 信息 ID: 033aaff76375d5aed899ff97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$1 <- core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$1 <- core::ptr::drop_in_place<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$1 <- country_core::systems::ui::resources::widget_params::WidgetParams::signifier<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; module=2235, inline_record=41224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_decorator::ui_color_windows_and_flags::closure$2
// 信息 ID: 1970634a03d7ad0a15a957db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::ui_color_decorator::ui_color_windows_and_flags", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_decorator::ui_color_windows_and_flags; module=3479, inline_record=1788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
