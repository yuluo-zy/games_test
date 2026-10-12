// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/color/src/write_roof_color_id_to_gpu.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 23ccae422e6aea273110d11acb10a1afebd5284f012f3ca70f02b252469aa339
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: write_roof_color_id_to_gpu.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3472
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::write_roof_color_id_to_gpu::write_roof_color_id_to_gpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3472,"record_offset":1372,"rva":27968384,"va":5396677504}
// 编译实例 RVAs: 0x1aac380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: write_roof_color_id_to_gpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26864623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396677504,"raw_hex":"e8111466fe","size":5,"soft_fail":false,"target_va":5369813552,"va":5396678170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: write_roof_color_id_to_gpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24047749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396677504,"raw_hex":"e87b0f91fe","size":5,"soft_fail":false,"target_va":5372630832,"va":5396678576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065ad20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: write_roof_color_id_to_gpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21305587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396677504,"raw_hex":"e80de7bafe","size":5,"soft_fail":false,"target_va":5375372576,"va":5396678158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: write_roof_color_id_to_gpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20391986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396677504,"raw_hex":"e8ced7c8fe","size":5,"soft_fail":false,"target_va":5376286288,"va":5396678269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dc81e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: write_roof_color_id_to_gpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3259938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396677504,"raw_hex":"e822be3100","size":5,"soft_fail":false,"target_va":5399937504,"va":5396677561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: write_roof_color_id_to_gpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6915714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396677504,"raw_hex":"e882866900","size":5,"soft_fail":false,"target_va":5403593344,"va":5396677625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6300352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396677504,"raw_hex":"e8c0226000","size":5,"soft_fail":false,"target_va":5402978880,"va":5396678523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6736060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396677504,"raw_hex":"e8bcc86600","size":5,"soft_fail":false,"target_va":5403414576,"va":5396678511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6301656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396677504,"raw_hex":"e8d8276000","size":5,"soft_fail":false,"target_va":5402980128,"va":5396678467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5396677504,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5396677645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3472,"record_offset":1816,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3472,"record_offset":2904,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3472,"record_offset":1780,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3472,"record_offset":3948,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3472,"record_offset":1544,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write_element
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3472,"record_offset":3808,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":106661,"source_line":198,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3472,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":1372,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3472,"record_offset":1880,"parent_record_offset":1840,"ancestor_record_offset":1372,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3472,"record_offset":3056,"parent_record_offset":3028,"ancestor_record_offset":1372,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3472,"record_offset":3084,"parent_record_offset":3056,"ancestor_record_offset":1372,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3472,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":1372,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3472,"record_offset":1612,"parent_record_offset":1572,"ancestor_record_offset":1372,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3472,"record_offset":2936,"parent_record_offset":2904,"ancestor_record_offset":1372,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3472,"record_offset":3112,"parent_record_offset":3084,"ancestor_record_offset":1372,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3472,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":1372,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3472,"record_offset":3028,"parent_record_offset":3000,"ancestor_record_offset":1372,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3472,"record_offset":1944,"parent_record_offset":1904,"ancestor_record_offset":1372,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3472,"record_offset":3000,"parent_record_offset":2936,"ancestor_record_offset":1372,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3472,"record_offset":2968,"parent_record_offset":2936,"ancestor_record_offset":1372,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3472,"record_offset":3196,"parent_record_offset":3112,"ancestor_record_offset":1372,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3472,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":1372,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3472,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":1372,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3472,"record_offset":3900,"parent_record_offset":3864,"ancestor_record_offset":1372,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3472,"record_offset":1696,"parent_record_offset":1668,"ancestor_record_offset":1372,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3472,"record_offset":1724,"parent_record_offset":1696,"ancestor_record_offset":1372,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3472,"record_offset":2008,"parent_record_offset":1904,"ancestor_record_offset":1372,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3472,"record_offset":2196,"parent_record_offset":1904,"ancestor_record_offset":1372,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3472,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":1372,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3472,"record_offset":2044,"parent_record_offset":2008,"ancestor_record_offset":1372,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3472,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1372,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3472,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":1372,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3472,"record_offset":3272,"parent_record_offset":3220,"ancestor_record_offset":1372,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3472,"record_offset":1640,"parent_record_offset":1612,"ancestor_record_offset":1372,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3472,"record_offset":1572,"parent_record_offset":1544,"ancestor_record_offset":1372,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3472,"record_offset":3836,"parent_record_offset":3808,"ancestor_record_offset":1372,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/03472.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3472,"record_offset":3864,"parent_record_offset":3836,"ancestor_record_offset":1372,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/03472.jsonl.gz"}
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
// 完整结构及原始类型记录: write_roof_color_id_to_gpu.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: write_roof_color_id_to_gpu.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: write_roof_color_id_to_gpu.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::EventReader<system_color::set_roof_color_id::SetRoofColorIdCmd>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<system_color::set_roof_color_id::SetRoofColorIdCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>

// ------------------------------------------------------------------
// 函数: system_color::write_roof_color_id_to_gpu::write_roof_color_id_to_gpu
// 信息 ID: 8352b5f1db5c5ee3ddc76365
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 163c1d840b2b7139dc2a7c9a: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>)> > @ 0x149bf8f -> 0x1aac380 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>)> @ 0x14b4b64 -> 0x1aac380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
