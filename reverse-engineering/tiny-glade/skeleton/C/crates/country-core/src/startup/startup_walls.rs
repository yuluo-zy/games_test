// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/startup_walls.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d7f90bd463d57316a15566559298631c815d8a8799245bde3ede80a3d13116a9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_walls.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 578, 1231, 1643, 1682, 1687, 1698
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use mesh_to_sdf; // namespace candidate
//   package identities: [{"name":"mesh_to_sdf","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_walls::glade_startup_walls
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":578,"record_offset":2216,"rva":12200848,"va":5380909968}
// 编译实例 RVAs: 0xba2b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3698581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8956f3800","size":5,"soft_fail":false,"target_va":5384609408,"va":5380910822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-5233166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8f225b0ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380911849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc_zeroed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5232389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8fb28b0ff","size":5,"soft_fail":false,"target_va":5375678736,"va":5380911120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-5233712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8d023b0ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380912411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12188920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8080346ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380911107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11252402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e84e4d54ff","size":5,"soft_fail":false,"target_va":5369660368,"va":5380912765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140103f50
//     owner=unresolved_namespace; count=2
//     target_aliases=17 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11143311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e871f755ff","size":5,"soft_fail":false,"target_va":5369773904,"va":5380917210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2785140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e88c80d5ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5380912079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096eb80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2315752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e818aadcff","size":5,"soft_fail":false,"target_va":5378599808,"va":5380915555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f9c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2182319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e851b3deff","size":5,"soft_fail":false,"target_va":5378734528,"va":5380916842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2182634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e816b2deff","size":5,"soft_fail":false,"target_va":5378736416,"va":5380919045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140991e20
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2173946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e806d4deff","size":5,"soft_fail":false,"target_va":5378743840,"va":5380917781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140992ce0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2167180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e874eedeff","size":5,"soft_fail":false,"target_va":5378747616,"va":5380914791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140996d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2153733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8fb22dfff","size":5,"soft_fail":false,"target_va":5378764144,"va":5380917872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a22f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1573030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e85affe7ff","size":5,"soft_fail":false,"target_va":5379338064,"va":5380911089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t664495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8af230a00","size":5,"soft_fail":false,"target_va":5381577744,"va":5380913244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c69cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t813914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e85a6b0c00","size":5,"soft_fail":false,"target_va":5381725360,"va":5380911441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2414402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e842d72400","size":5,"soft_fail":false,"target_va":5383324784,"va":5380910377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14156bc00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10256807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8a7819c00","size":5,"soft_fail":false,"target_va":5391170560,"va":5380913748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415c12e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10606503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8a7d7a100","size":5,"soft_fail":false,"target_va":5391520480,"va":5380913972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415e7450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10764707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8a341a400","size":5,"soft_fail":false,"target_va":5391676496,"va":5380911784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141603ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10882961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8910fa600","size":5,"soft_fail":false,"target_va":5391793856,"va":5380910890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141605350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10887053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e88d1fa600","size":5,"soft_fail":false,"target_va":5391799120,"va":5380912062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162d150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11046974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e83e90a800","size":5,"soft_fail":false,"target_va":5391962448,"va":5380915469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141630e00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11063636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e854d1a800","size":5,"soft_fail":false,"target_va":5391977984,"va":5380914343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aed3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16033720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8b8a7f400","size":5,"soft_fail":false,"target_va":5396943824,"va":5380910099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: startup_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36146403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8e38c2702","size":5,"soft_fail":false,"target_va":5417057984,"va":5380911576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35704999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8a7d02002","size":5,"soft_fail":false,"target_va":5416620769,"va":5380915765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35709644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8cce22002","size":5,"soft_fail":false,"target_va":5416620794,"va":5380911145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3704932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e864883800","size":5,"soft_fail":false,"target_va":5384624080,"va":5380919143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t35705951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e85fd42002","size":5,"soft_fail":false,"target_va":5416621600,"va":5380915644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35705722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e87ad32002","size":5,"soft_fail":false,"target_va":5416621762,"va":5380916035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<array$<rhapsody::raw::binding_set::BindingSet,2> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t55489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8c1d80000","size":5,"soft_fail":false,"target_va":5380974096,"va":5380918602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35705890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e822d42002","size":5,"soft_fail":false,"target_va":5416621632,"va":5380915737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_world_raster_pipelines
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1391308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e834c5eaff","size":5,"soft_fail":false,"target_va":5379521760,"va":5380913063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::add
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1629760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8c021e7ff","size":5,"soft_fail":false,"target_va":5379282224,"va":5380911979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name_hash
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1634642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8ae0ee7ff","size":5,"soft_fail":false,"target_va":5379278928,"va":5380913565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t556096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8407c0800","size":5,"soft_fail":false,"target_va":5381466592,"va":5380910491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t554561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e841760800","size":5,"soft_fail":false,"target_va":5381465408,"va":5380910842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1263000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e868baecff","size":5,"soft_fail":false,"target_va":5379649664,"va":5380912659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::binding_set::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_binding_set
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t2789520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e890902a00","size":5,"soft_fail":false,"target_va":5383702992,"va":5380913467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35703242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8cac92002","size":5,"soft_fail":false,"target_va":5416619360,"va":5380916113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::expand_to_contain_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2391933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e87d7f2400","size":5,"soft_fail":false,"target_va":5383302544,"va":5380910606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2411182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380909968,"raw_hex":"e8aeca2400","size":5,"soft_fail":false,"target_va":5383324736,"va":5380913549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5380909968,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5380913266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":4280,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":4692,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":4788,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":578,"record_offset":5068,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":578,"record_offset":8944,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":578,"record_offset":3908,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":578,"record_offset":4884,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":578,"record_offset":3532,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":578,"record_offset":13856,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":578,"record_offset":3504,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":578,"record_offset":4076,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":578,"record_offset":9088,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":578,"record_offset":8812,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":6816,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":578,"record_offset":2492,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":578,"record_offset":5308,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":21763,"source_line":34,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":578,"record_offset":5280,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::shader_library::render_pipelines_impl::impl$2::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/shader_library.rs
//     evidence={"module":578,"record_offset":9600,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":32867,"source_line":61,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":578,"record_offset":8752,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":578,"record_offset":9356,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_handle_by_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":578,"record_offset":2568,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":31917,"source_line":308,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":578,"record_offset":5124,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_walls::create_half_cube_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_walls.rs
//     evidence={"module":578,"record_offset":7908,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":36799,"source_line":405,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_walls::glade_startup_walls::f32_to_r8_snorm
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_walls.rs
//     evidence={"module":578,"record_offset":4916,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":36796,"source_line":97,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] itertools::adaptors::map::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/adaptors/map.rs
//     evidence={"module":578,"record_offset":4632,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":36794,"source_line":23,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":578,"record_offset":3964,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":578,"record_offset":3824,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] mesh_to_sdf::generate::grid::generate_grid_sdf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"mesh_to_sdf","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/mesh_to_sdf-0.4.0/src/generate/grid.rs
//     evidence={"module":578,"record_offset":4252,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":32000,"source_line":265,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] mesh_to_sdf::grid::Grid::from_bounding_box
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"mesh_to_sdf","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/mesh_to_sdf-0.4.0/src/grid.rs
//     evidence={"module":578,"record_offset":4160,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":36790,"source_line":59,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":578,"record_offset":9448,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":27992,"source_line":406,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::impl$43::clone
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":578,"record_offset":11392,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":30000,"source_line":78,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::create_texture
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":578,"record_offset":5208,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":14907,"source_line":881,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureDesc::new_cube
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":578,"record_offset":6084,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":36797,"source_line":488,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureDesc::sampled
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":578,"record_offset":5180,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":9289,"source_line":512,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":4496,"parent_record_offset":4472,"ancestor_record_offset":2216,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":4520,"parent_record_offset":4496,"ancestor_record_offset":2216,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":2216,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":2216,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":2216,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":4544,"parent_record_offset":4520,"ancestor_record_offset":2216,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":2216,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":5644,"parent_record_offset":5616,"ancestor_record_offset":2216,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":4472,"parent_record_offset":4428,"ancestor_record_offset":2216,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":6944,"parent_record_offset":6888,"ancestor_record_offset":2216,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":578,"record_offset":7992,"parent_record_offset":7964,"ancestor_record_offset":2216,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":578,"record_offset":7964,"parent_record_offset":7908,"ancestor_record_offset":2216,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":5556,"parent_record_offset":5528,"ancestor_record_offset":2216,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":4352,"parent_record_offset":4316,"ancestor_record_offset":2216,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":6916,"parent_record_offset":6888,"ancestor_record_offset":2216,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":6888,"parent_record_offset":6864,"ancestor_record_offset":2216,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":4428,"parent_record_offset":4388,"ancestor_record_offset":2216,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":5584,"parent_record_offset":5556,"ancestor_record_offset":2216,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":4388,"parent_record_offset":4352,"ancestor_record_offset":2216,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":2216,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":578,"record_offset":5436,"parent_record_offset":5408,"ancestor_record_offset":2216,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":578,"record_offset":5464,"parent_record_offset":5436,"ancestor_record_offset":2216,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":578,"record_offset":5492,"parent_record_offset":5464,"ancestor_record_offset":2216,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":578,"record_offset":5408,"parent_record_offset":5380,"ancestor_record_offset":2216,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":578,"record_offset":5380,"parent_record_offset":5352,"ancestor_record_offset":2216,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":578,"record_offset":5352,"parent_record_offset":5308,"ancestor_record_offset":2216,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":578,"record_offset":5148,"parent_record_offset":5124,"ancestor_record_offset":2216,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":5528,"parent_record_offset":5492,"ancestor_record_offset":2216,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":2216,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$2::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":578,"record_offset":4316,"parent_record_offset":4280,"ancestor_record_offset":2216,"inlinee":36792,"source_line":34,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":578,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":2216,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":578,"record_offset":8968,"parent_record_offset":8944,"ancestor_record_offset":2216,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":578,"record_offset":8384,"parent_record_offset":8356,"ancestor_record_offset":2216,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":578,"record_offset":8436,"parent_record_offset":8412,"ancestor_record_offset":2216,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":578,"record_offset":8412,"parent_record_offset":8356,"ancestor_record_offset":2216,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":578,"record_offset":13912,"parent_record_offset":13884,"ancestor_record_offset":2216,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$77::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":578,"record_offset":5032,"parent_record_offset":4916,"ancestor_record_offset":2216,"inlinee":36787,"source_line":2016,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":578,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":2216,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":578,"record_offset":2796,"parent_record_offset":2772,"ancestor_record_offset":2216,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":578,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":2216,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":578,"record_offset":4944,"parent_record_offset":4916,"ancestor_record_offset":2216,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":578,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":2216,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":578,"record_offset":3328,"parent_record_offset":2692,"ancestor_record_offset":2216,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":578,"record_offset":3560,"parent_record_offset":3532,"ancestor_record_offset":2216,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":578,"record_offset":13884,"parent_record_offset":13856,"ancestor_record_offset":2216,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":578,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":2216,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":578,"record_offset":8460,"parent_record_offset":8436,"ancestor_record_offset":2216,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":578,"record_offset":9044,"parent_record_offset":9020,"ancestor_record_offset":2216,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":578,"record_offset":7456,"parent_record_offset":7428,"ancestor_record_offset":2216,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":578,"record_offset":9828,"parent_record_offset":9800,"ancestor_record_offset":2216,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":578,"record_offset":3028,"parent_record_offset":3000,"ancestor_record_offset":2216,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":578,"record_offset":3000,"parent_record_offset":2972,"ancestor_record_offset":2216,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":578,"record_offset":3744,"parent_record_offset":3712,"ancestor_record_offset":2216,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":578,"record_offset":9020,"parent_record_offset":8992,"ancestor_record_offset":2216,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":578,"record_offset":8992,"parent_record_offset":8968,"ancestor_record_offset":2216,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":578,"record_offset":3680,"parent_record_offset":3560,"ancestor_record_offset":2216,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":578,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":2216,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":578,"record_offset":3712,"parent_record_offset":3532,"ancestor_record_offset":2216,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":578,"record_offset":9992,"parent_record_offset":9628,"ancestor_record_offset":2216,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":578,"record_offset":8328,"parent_record_offset":8300,"ancestor_record_offset":2216,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":578,"record_offset":5772,"parent_record_offset":5492,"ancestor_record_offset":2216,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":2216,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":2536,"parent_record_offset":2492,"ancestor_record_offset":2216,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":578,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":2216,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":578,"record_offset":9800,"parent_record_offset":9776,"ancestor_record_offset":2216,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":578,"record_offset":9856,"parent_record_offset":9800,"ancestor_record_offset":2216,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":578,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":2216,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":3648,"parent_record_offset":3588,"ancestor_record_offset":2216,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":9752,"parent_record_offset":9728,"ancestor_record_offset":2216,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":9776,"parent_record_offset":9752,"ancestor_record_offset":2216,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":578,"record_offset":3620,"parent_record_offset":3588,"ancestor_record_offset":2216,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":578,"record_offset":4720,"parent_record_offset":4692,"ancestor_record_offset":2216,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":578,"record_offset":4816,"parent_record_offset":4788,"ancestor_record_offset":2216,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":578,"record_offset":4748,"parent_record_offset":4720,"ancestor_record_offset":2216,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":578,"record_offset":4844,"parent_record_offset":4816,"ancestor_record_offset":2216,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":578,"record_offset":3588,"parent_record_offset":3560,"ancestor_record_offset":2216,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":578,"record_offset":9928,"parent_record_offset":9700,"ancestor_record_offset":2216,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":578,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":2216,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":578,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":2216,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":578,"record_offset":3872,"parent_record_offset":3848,"ancestor_record_offset":2216,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":578,"record_offset":8776,"parent_record_offset":8752,"ancestor_record_offset":2216,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$7::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":578,"record_offset":9628,"parent_record_offset":9600,"ancestor_record_offset":2216,"inlinee":32855,"source_line":17,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":578,"record_offset":8148,"parent_record_offset":7908,"ancestor_record_offset":2216,"inlinee":30455,"source_line":40,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$1::positions::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":578,"record_offset":3776,"parent_record_offset":3744,"ancestor_record_offset":2216,"inlinee":36789,"source_line":196,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":578,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":2216,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":578,"record_offset":2972,"parent_record_offset":2940,"ancestor_record_offset":2216,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":578,"record_offset":2912,"parent_record_offset":2692,"ancestor_record_offset":2216,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":578,"record_offset":2772,"parent_record_offset":2692,"ancestor_record_offset":2216,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":578,"record_offset":3356,"parent_record_offset":2692,"ancestor_record_offset":2216,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":578,"record_offset":2856,"parent_record_offset":2692,"ancestor_record_offset":2216,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":578,"record_offset":3264,"parent_record_offset":3240,"ancestor_record_offset":2216,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":578,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":2216,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":578,"record_offset":3240,"parent_record_offset":3072,"ancestor_record_offset":2216,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":3180,"parent_record_offset":3096,"ancestor_record_offset":2216,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":3124,"parent_record_offset":3096,"ancestor_record_offset":2216,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":3448,"parent_record_offset":2692,"ancestor_record_offset":2216,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":2216,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":2668,"parent_record_offset":2644,"ancestor_record_offset":2216,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":2644,"parent_record_offset":2616,"ancestor_record_offset":2216,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":2216,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":2744,"parent_record_offset":2692,"ancestor_record_offset":2216,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":578,"record_offset":3072,"parent_record_offset":2692,"ancestor_record_offset":2216,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":578,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":2216,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":578,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":2216,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":578,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":2216,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] mesh_to_sdf::point::Point::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"mesh_to_sdf","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/mesh_to_sdf-0.4.0/src/point.rs
//     evidence={"module":578,"record_offset":4192,"parent_record_offset":4160,"ancestor_record_offset":2216,"inlinee":31992,"source_line":81,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] mesh_to_sdf::point::impl_array::impl$0::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"mesh_to_sdf","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/mesh_to_sdf-0.4.0/src/point/impl_array.rs
//     evidence={"module":578,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":2216,"inlinee":31882,"source_line":7,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding_set::impl$26::clone
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":578,"record_offset":11064,"parent_record_offset":11036,"ancestor_record_offset":2216,"inlinee":16899,"source_line":78,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=18
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":578,"record_offset":9728,"parent_record_offset":9700,"ancestor_record_offset":2216,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=18
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":578,"record_offset":9700,"parent_record_offset":9676,"ancestor_record_offset":2216,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$44::clone
//     owner=git_dependency_definition; count=12
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":578,"record_offset":9676,"parent_record_offset":9628,"ancestor_record_offset":2216,"inlinee":32857,"source_line":78,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":578,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":2216,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":578,"record_offset":8184,"parent_record_offset":8148,"ancestor_record_offset":2216,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":578,"record_offset":4976,"parent_record_offset":4916,"ancestor_record_offset":2216,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":578,"record_offset":8240,"parent_record_offset":8212,"ancestor_record_offset":2216,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":578,"record_offset":8356,"parent_record_offset":8300,"ancestor_record_offset":2216,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":578,"record_offset":8212,"parent_record_offset":8184,"ancestor_record_offset":2216,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":578,"record_offset":8300,"parent_record_offset":8268,"ancestor_record_offset":2216,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":578,"record_offset":8268,"parent_record_offset":8240,"ancestor_record_offset":2216,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_walls::once_startup_walls
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8FED
// 首条位置: {"module":1231,"inlinee":36845}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1231,"record_offset":61604,"parent_record_offset":61560,"ancestor_record_offset":61560,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1231,"record_offset":61744,"parent_record_offset":61560,"ancestor_record_offset":61560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01231.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::subscribe
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1231,"record_offset":61716,"parent_record_offset":61560,"ancestor_record_offset":61560,"inlinee":26553,"source_line":248,"archive":"_pdb/C/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1231,"record_offset":61792,"parent_record_offset":61768,"ancestor_record_offset":61560,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1231,"record_offset":61628,"parent_record_offset":61604,"ancestor_record_offset":61560,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1231,"record_offset":61768,"parent_record_offset":61744,"ancestor_record_offset":61560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1231,"record_offset":61828,"parent_record_offset":61792,"ancestor_record_offset":61560,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1231,"record_offset":61852,"parent_record_offset":61828,"ancestor_record_offset":61560,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01231.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1231,"record_offset":61884,"parent_record_offset":61792,"ancestor_record_offset":61560,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01231.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_walls::glade_startup_walls::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1436D
// 首条位置: {"module":1682,"inlinee":82797}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$22::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1687,"record_offset":1760,"parent_record_offset":1544,"ancestor_record_offset":1544,"inlinee":82799,"source_line":1484,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1687,"record_offset":1672,"parent_record_offset":1544,"ancestor_record_offset":1544,"inlinee":49085,"source_line":1309,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1687,"record_offset":1580,"parent_record_offset":1544,"ancestor_record_offset":1544,"inlinee":26700,"source_line":1764,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1687,"record_offset":1700,"parent_record_offset":1672,"ancestor_record_offset":1544,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1687,"record_offset":1612,"parent_record_offset":1580,"ancestor_record_offset":1544,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1687,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":1544,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/01687.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_walls::glade_startup_walls::f32_to_r8_snorm
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8FBC
// 首条位置: {"module":578,"inlinee":36796}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$77::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":578,"record_offset":5032,"parent_record_offset":4916,"ancestor_record_offset":4916,"inlinee":36787,"source_line":2016,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":578,"record_offset":4944,"parent_record_offset":4916,"ancestor_record_offset":4916,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":578,"record_offset":4976,"parent_record_offset":4916,"ancestor_record_offset":4916,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":578,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4916,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_walls::glade_startup_walls::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D24
// 首条位置: {"module":1643,"inlinee":81188}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1643,"record_offset":96992,"parent_record_offset":96952,"ancestor_record_offset":96952,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":98612,"parent_record_offset":98588,"ancestor_record_offset":96952,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":98636,"parent_record_offset":98612,"ancestor_record_offset":96952,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":97768,"parent_record_offset":97744,"ancestor_record_offset":96952,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":97792,"parent_record_offset":97768,"ancestor_record_offset":96952,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":98660,"parent_record_offset":98636,"ancestor_record_offset":96952,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":97816,"parent_record_offset":97792,"ancestor_record_offset":96952,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":98588,"parent_record_offset":98444,"ancestor_record_offset":96952,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":97744,"parent_record_offset":97688,"ancestor_record_offset":96952,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":98884,"parent_record_offset":98848,"ancestor_record_offset":96952,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":98384,"parent_record_offset":98356,"ancestor_record_offset":96952,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":97716,"parent_record_offset":97688,"ancestor_record_offset":96952,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":97688,"parent_record_offset":97664,"ancestor_record_offset":96952,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::grow_amortized
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":98996,"parent_record_offset":98956,"ancestor_record_offset":96952,"inlinee":13392,"source_line":501,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":98920,"parent_record_offset":98884,"ancestor_record_offset":96952,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":98444,"parent_record_offset":98412,"ancestor_record_offset":96952,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":98412,"parent_record_offset":98384,"ancestor_record_offset":96952,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":97664,"parent_record_offset":97640,"ancestor_record_offset":96952,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":98956,"parent_record_offset":98920,"ancestor_record_offset":96952,"inlinee":13405,"source_line":659,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":98476,"parent_record_offset":98444,"ancestor_record_offset":96952,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":98784,"parent_record_offset":98748,"ancestor_record_offset":96952,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":98848,"parent_record_offset":98784,"ancestor_record_offset":96952,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":98328,"parent_record_offset":97140,"ancestor_record_offset":96952,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":98356,"parent_record_offset":98328,"ancestor_record_offset":96952,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":97032,"parent_record_offset":96992,"ancestor_record_offset":96952,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1643,"record_offset":97568,"parent_record_offset":97544,"ancestor_record_offset":96952,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1643,"record_offset":97616,"parent_record_offset":97592,"ancestor_record_offset":96952,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1643,"record_offset":98748,"parent_record_offset":97140,"ancestor_record_offset":96952,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1643,"record_offset":97100,"parent_record_offset":97032,"ancestor_record_offset":96952,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1643,"record_offset":97140,"parent_record_offset":97100,"ancestor_record_offset":96952,"inlinee":10962,"source_line":18,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1643,"record_offset":98500,"parent_record_offset":98476,"ancestor_record_offset":96952,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":98296,"parent_record_offset":98264,"ancestor_record_offset":96952,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":99144,"parent_record_offset":98996,"ancestor_record_offset":96952,"inlinee":10723,"source_line":1681,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1643,"record_offset":98548,"parent_record_offset":98524,"ancestor_record_offset":96952,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1643,"record_offset":97240,"parent_record_offset":97216,"ancestor_record_offset":96952,"inlinee":10912,"source_line":531,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$3::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1643,"record_offset":97216,"parent_record_offset":97140,"ancestor_record_offset":96952,"inlinee":10911,"source_line":68,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1643,"record_offset":97432,"parent_record_offset":97240,"ancestor_record_offset":96952,"inlinee":10913,"source_line":75,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1643,"record_offset":97072,"parent_record_offset":97032,"ancestor_record_offset":96952,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1643,"record_offset":99088,"parent_record_offset":98996,"ancestor_record_offset":96952,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1643,"record_offset":98524,"parent_record_offset":98500,"ancestor_record_offset":96952,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1643,"record_offset":97404,"parent_record_offset":97240,"ancestor_record_offset":96952,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1643,"record_offset":97276,"parent_record_offset":97240,"ancestor_record_offset":96952,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1643,"record_offset":97308,"parent_record_offset":97240,"ancestor_record_offset":96952,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":97472,"parent_record_offset":97140,"ancestor_record_offset":96952,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":98720,"parent_record_offset":97140,"ancestor_record_offset":96952,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1643,"record_offset":99044,"parent_record_offset":98996,"ancestor_record_offset":96952,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_walls::glade_startup_walls::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D2B
// 首条位置: {"module":1643,"inlinee":81195}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":118248,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":120848,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":119476,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1643,"record_offset":118540,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":122264,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1643,"record_offset":120736,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$90::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1643,"record_offset":120792,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":11952,"source_line":1896,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_walls::glade_startup_walls::f32_to_r8_snorm
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_walls.rs
//     evidence={"module":1643,"record_offset":118932,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":36796,"source_line":97,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":1643,"record_offset":119604,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":9824,"source_line":893,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":120284,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":11808,"source_line":466,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::max_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":120592,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":14575,"source_line":340,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":118684,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":120020,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":120192,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":9816,"source_line":1553,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":121160,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":120048,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":121436,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":120500,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":120648,"parent_record_offset":118024,"ancestor_record_offset":118024,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":118432,"parent_record_offset":118408,"ancestor_record_offset":118024,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":118456,"parent_record_offset":118432,"ancestor_record_offset":118024,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":122384,"parent_record_offset":122360,"ancestor_record_offset":118024,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":122408,"parent_record_offset":122384,"ancestor_record_offset":118024,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":118480,"parent_record_offset":118456,"ancestor_record_offset":118024,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":122432,"parent_record_offset":122408,"ancestor_record_offset":118024,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":118408,"parent_record_offset":118376,"ancestor_record_offset":118024,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":122360,"parent_record_offset":122336,"ancestor_record_offset":118024,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":118312,"parent_record_offset":118280,"ancestor_record_offset":118024,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":122336,"parent_record_offset":122312,"ancestor_record_offset":118024,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":118376,"parent_record_offset":118344,"ancestor_record_offset":118024,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":118344,"parent_record_offset":118312,"ancestor_record_offset":118024,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":122312,"parent_record_offset":122288,"ancestor_record_offset":118024,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1643,"record_offset":118280,"parent_record_offset":118248,"ancestor_record_offset":118024,"inlinee":9075,"source_line":49,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":118592,"parent_record_offset":118564,"ancestor_record_offset":118024,"inlinee":22737,"source_line":1916,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$77::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":119060,"parent_record_offset":118932,"ancestor_record_offset":118024,"inlinee":36787,"source_line":2016,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1643,"record_offset":120344,"parent_record_offset":120320,"ancestor_record_offset":118024,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1643,"record_offset":118968,"parent_record_offset":118932,"ancestor_record_offset":118024,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1643,"record_offset":119028,"parent_record_offset":119004,"ancestor_record_offset":118024,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1643,"record_offset":118828,"parent_record_offset":118804,"ancestor_record_offset":118024,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$37::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1643,"record_offset":118620,"parent_record_offset":118564,"ancestor_record_offset":118024,"inlinee":39800,"source_line":211,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1643,"record_offset":118564,"parent_record_offset":118540,"ancestor_record_offset":118024,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1643,"record_offset":119564,"parent_record_offset":119532,"ancestor_record_offset":118024,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1643,"record_offset":118644,"parent_record_offset":118620,"ancestor_record_offset":118024,"inlinee":39801,"source_line":832,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":119728,"parent_record_offset":119684,"ancestor_record_offset":118024,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":118868,"parent_record_offset":118708,"ancestor_record_offset":118024,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":120676,"parent_record_offset":120648,"ancestor_record_offset":118024,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":120532,"parent_record_offset":120500,"ancestor_record_offset":118024,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":120076,"parent_record_offset":120048,"ancestor_record_offset":118024,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":121468,"parent_record_offset":121436,"ancestor_record_offset":118024,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":122288,"parent_record_offset":122264,"ancestor_record_offset":118024,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1643,"record_offset":120816,"parent_record_offset":120792,"ancestor_record_offset":118024,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1643,"record_offset":120876,"parent_record_offset":120848,"ancestor_record_offset":118024,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1643,"record_offset":119504,"parent_record_offset":119476,"ancestor_record_offset":118024,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1643,"record_offset":120904,"parent_record_offset":120876,"ancestor_record_offset":118024,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1643,"record_offset":119532,"parent_record_offset":119504,"ancestor_record_offset":118024,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1643,"record_offset":120760,"parent_record_offset":120736,"ancestor_record_offset":118024,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":1643,"record_offset":119636,"parent_record_offset":119604,"ancestor_record_offset":118024,"inlinee":9826,"source_line":625,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1643,"record_offset":120320,"parent_record_offset":120284,"ancestor_record_offset":118024,"inlinee":11809,"source_line":161,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":118772,"parent_record_offset":118740,"ancestor_record_offset":118024,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":118740,"parent_record_offset":118708,"ancestor_record_offset":118024,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":118708,"parent_record_offset":118684,"ancestor_record_offset":118024,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$0::max_element::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":120616,"parent_record_offset":120592,"ancestor_record_offset":118024,"inlinee":14577,"source_line":341,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":119820,"parent_record_offset":119636,"ancestor_record_offset":118024,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":119684,"parent_record_offset":119636,"ancestor_record_offset":118024,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1643,"record_offset":119004,"parent_record_offset":118932,"ancestor_record_offset":118024,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1643,"record_offset":118804,"parent_record_offset":118772,"ancestor_record_offset":118024,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_walls::create_half_cube_mesh
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 405
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8FBF
// 首条位置: {"module":578,"inlinee":36799}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":578,"record_offset":7964,"parent_record_offset":7908,"ancestor_record_offset":7908,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":8524,"parent_record_offset":7908,"ancestor_record_offset":7908,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":578,"record_offset":8148,"parent_record_offset":7908,"ancestor_record_offset":7908,"inlinee":30455,"source_line":40,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":7908,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":7908,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":8644,"parent_record_offset":8620,"ancestor_record_offset":7908,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":8668,"parent_record_offset":8644,"ancestor_record_offset":7908,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":7908,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":7908,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":8024,"parent_record_offset":7992,"ancestor_record_offset":7908,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":8620,"parent_record_offset":8596,"ancestor_record_offset":7908,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":578,"record_offset":7992,"parent_record_offset":7964,"ancestor_record_offset":7908,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":8596,"parent_record_offset":8572,"ancestor_record_offset":7908,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":578,"record_offset":8572,"parent_record_offset":8548,"ancestor_record_offset":7908,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":578,"record_offset":8384,"parent_record_offset":8356,"ancestor_record_offset":7908,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":578,"record_offset":8436,"parent_record_offset":8412,"ancestor_record_offset":7908,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":578,"record_offset":8412,"parent_record_offset":8356,"ancestor_record_offset":7908,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":578,"record_offset":8460,"parent_record_offset":8436,"ancestor_record_offset":7908,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":578,"record_offset":8328,"parent_record_offset":8300,"ancestor_record_offset":7908,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":8548,"parent_record_offset":8524,"ancestor_record_offset":7908,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":578,"record_offset":8184,"parent_record_offset":8148,"ancestor_record_offset":7908,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":578,"record_offset":8240,"parent_record_offset":8212,"ancestor_record_offset":7908,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":578,"record_offset":8356,"parent_record_offset":8300,"ancestor_record_offset":7908,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":578,"record_offset":8212,"parent_record_offset":8184,"ancestor_record_offset":7908,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":578,"record_offset":8300,"parent_record_offset":8268,"ancestor_record_offset":7908,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":578,"record_offset":8268,"parent_record_offset":8240,"ancestor_record_offset":7908,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00578.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_walls.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_walls.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_walls.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 309; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_walls::create_half_cube_mesh
// 信息 ID: 64a764f25834de0aee0063c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_walls::glade_startup_walls; module=578, inline_record=7908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_walls::glade_startup_walls
// 信息 ID: b9c0e9b6e7e38f6b85fd43bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 88f775e4a01cbf39e082785f: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)> @ 0x116b3ee -> 0xba2b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)> > @ 0x1940f5a -> 0xba2b90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_walls::glade_startup_walls::closure$0
// 信息 ID: 8203edec0c1d2f1a4d365e73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::startup_walls::glade_startup_walls", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::startup_walls::glade_startup_walls::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,country_core::startup::startup_walls::glade_startup_walls::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::startup_walls::glade_startup_walls::closure_env$0 <- core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<array$<f32,3> >,array$<f32,3>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<array$<f32,3>,glam::f32::vec3::Vec3,tuple$<>,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<glam::f32::vec3::Vec3,array$<f32,3>,tuple$<>,country_core::startup::startup_walls::glade_startup_walls::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$21::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,country_core::startup::startup_walls::glade_startup_walls::closure_env$0> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::startup_walls::glade_startup_walls::closure_env$0 <- core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<array$<f32,3> >,array$<f32,3>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<array$<f32,3>,glam::f32::vec3::Vec3,tuple$<>,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<glam::f32::vec3::Vec3,array$<f32,3>,tuple$<>,country_core::startup::startup_walls::glade_startup_walls::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$21::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,country_core::startup::startup_walls::glade_startup_walls::closure_env$0> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<array$<f32,3> >,array$<f32,3>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<array$<f32,3>,glam::f32::vec3::Vec3,tuple$<>,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<glam::f32::vec3::Vec3,array$<f32,3>,tuple$<>,country_core::startup::startup_walls::glade_startup_walls::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$21::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,country_core::startup::startup_walls::glade_startup_walls::closure_env$0> > > > > >; module=1687, inline_record=1544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_walls::glade_startup_walls::closure$1
// 信息 ID: 26aa3dd372f670ed95a64d90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::startup_walls::glade_startup_walls", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::startup_walls::glade_startup_walls::closure_env$1 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::ops::range::Range<i32>,alloc::vec::Vec<u8,alloc::alloc::Global>,country_core::startup::startup_walls::glade_startup_walls::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::startup_walls::glade_startup_walls::closure_env$1 <- core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<i32>,alloc::vec::Vec<u8,alloc::alloc::Global>,country_core::startup::startup_walls::glade_startup_walls::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::startup_walls::glade_startup_walls::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<u8,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<i32>,alloc::vec::Vec<u8,alloc::alloc::Global>,country_core::startup::startup_walls::glade_startup_walls::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<i32>,alloc::vec::Vec<u8,alloc::alloc::Global>,country_core::startup::startup_walls::glade_startup_walls::closure_env$1>; module=1643, inline_record=96952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_walls::glade_startup_walls::closure$1::closure$0
// 信息 ID: f789ddacf20ba1cae40c4320
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::startup_walls::glade_startup_walls", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::startup_walls::glade_startup_walls::closure$1::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u16>,alloc::vec::Vec<u8,alloc::alloc::Global>,country_core::startup::startup_walls::glade_startup_walls::closure$1::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::startup_walls::glade_startup_walls::closure$1::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<u16>,alloc::vec::Vec<u8,alloc::alloc::Global>,country_core::startup::startup_walls::glade_startup_walls::closure$1::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<u16>,alloc::vec::Vec<u8,alloc::alloc::Global>,country_core::startup::startup_walls::glade_startup_walls::closure$1::closure_env$0>; module=1643, inline_record=118024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_walls::glade_startup_walls::f32_to_r8_snorm
// 信息 ID: 6c9aad58aedf533f6925f4d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_walls::glade_startup_walls; module=578, inline_record=4916; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<u16>,alloc::vec::Vec<u8,alloc::alloc::Global>,country_core::startup::startup_walls::glade_startup_walls::closure$1::closure_env$0>; module=1643, inline_record=118932; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<u16>,alloc::vec::Vec<u8,alloc::alloc::Global>,country_core::startup::startup_walls::glade_startup_walls::closure$1::closure_env$0>; module=1643, inline_record=119092; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<u16>,alloc::vec::Vec<u8,alloc::alloc::Global>,country_core::startup::startup_walls::glade_startup_walls::closure$1::closure_env$0>; module=1643, inline_record=119244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_walls::once_startup_walls
// 信息 ID: 52aa3cdfd4602c95b8d4a8f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fc68acc99a848e3e9f4a896d: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>; module=1231, inline_record=61560; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >; module=1698, inline_record=4345564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
