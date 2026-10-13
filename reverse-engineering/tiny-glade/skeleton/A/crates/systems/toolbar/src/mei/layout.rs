// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/mei/layout.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5c738ceef9577b05c958986ba4dd21ca00182cf7076703d2d804435b0027308e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: layout.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 8, 319, 320, 1025, 1046, 2447, 2475
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use ron; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1046,"record_offset":2328,"rva":11026176,"va":5379735296}
// 编译实例 RVAs: 0xa83f00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10122024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379735296,"raw_hex":"e8d88c65ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5379735539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23242579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379735296,"raw_hex":"e853a76201","size":5,"soft_fail":false,"target_va":5402978912,"va":5379736328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1046,"record_offset":4448,"parent_record_offset":2328,"ancestor_record_offset":2328,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1046,"record_offset":3140,"parent_record_offset":2328,"ancestor_record_offset":2328,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1046,"record_offset":2728,"parent_record_offset":2328,"ancestor_record_offset":2328,"inlinee":24559,"source_line":175,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1046,"record_offset":2872,"parent_record_offset":2328,"ancestor_record_offset":2328,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1046,"record_offset":6468,"parent_record_offset":2328,"ancestor_record_offset":2328,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1046,"record_offset":6380,"parent_record_offset":2328,"ancestor_record_offset":2328,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points_or_zero
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":2700,"parent_record_offset":2328,"ancestor_record_offset":2328,"inlinee":17448,"source_line":368,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::points_or_zero
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":6004,"parent_record_offset":2328,"ancestor_record_offset":2328,"inlinee":17470,"source_line":394,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::size_vec2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":6240,"parent_record_offset":2328,"ancestor_record_offset":2328,"inlinee":17453,"source_line":116,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Size::points_or_zero
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":5812,"parent_record_offset":2328,"ancestor_record_offset":2328,"inlinee":17451,"source_line":384,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1046,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":2328,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1046,"record_offset":3680,"parent_record_offset":3656,"ancestor_record_offset":2328,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1046,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":2328,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1046,"record_offset":6496,"parent_record_offset":6468,"ancestor_record_offset":2328,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1046,"record_offset":3704,"parent_record_offset":3680,"ancestor_record_offset":2328,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1046,"record_offset":2916,"parent_record_offset":2872,"ancestor_record_offset":2328,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1046,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":2328,"inlinee":24564,"source_line":145,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1046,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":2328,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1046,"record_offset":6288,"parent_record_offset":6264,"ancestor_record_offset":2328,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1046,"record_offset":3020,"parent_record_offset":2916,"ancestor_record_offset":2328,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1046,"record_offset":3264,"parent_record_offset":3168,"ancestor_record_offset":2328,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1046,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":2328,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1046,"record_offset":3296,"parent_record_offset":3140,"ancestor_record_offset":2328,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1046,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":2328,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1046,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":2328,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1046,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":2328,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1046,"record_offset":2804,"parent_record_offset":2776,"ancestor_record_offset":2328,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1046,"record_offset":2984,"parent_record_offset":2956,"ancestor_record_offset":2328,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$180::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1046,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":2328,"inlinee":7906,"source_line":141,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1046,"record_offset":2956,"parent_record_offset":2916,"ancestor_record_offset":2328,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1046,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":2328,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1046,"record_offset":3448,"parent_record_offset":3420,"ancestor_record_offset":2328,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1046,"record_offset":3476,"parent_record_offset":3448,"ancestor_record_offset":2328,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1046,"record_offset":4164,"parent_record_offset":3944,"ancestor_record_offset":2328,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1046,"record_offset":3508,"parent_record_offset":3476,"ancestor_record_offset":2328,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1046,"record_offset":4076,"parent_record_offset":4052,"ancestor_record_offset":2328,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1046,"record_offset":4052,"parent_record_offset":3944,"ancestor_record_offset":2328,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1046,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":2328,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1046,"record_offset":4372,"parent_record_offset":3584,"ancestor_record_offset":2328,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1046,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":2328,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1046,"record_offset":3560,"parent_record_offset":3536,"ancestor_record_offset":2328,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1046,"record_offset":3536,"parent_record_offset":3476,"ancestor_record_offset":2328,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1046,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":2328,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1046,"record_offset":3824,"parent_record_offset":3796,"ancestor_record_offset":2328,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1046,"record_offset":3852,"parent_record_offset":3824,"ancestor_record_offset":2328,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1046,"record_offset":3912,"parent_record_offset":3796,"ancestor_record_offset":2328,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1046,"record_offset":3796,"parent_record_offset":3584,"ancestor_record_offset":2328,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1046,"record_offset":3628,"parent_record_offset":3584,"ancestor_record_offset":2328,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1046,"record_offset":3944,"parent_record_offset":3584,"ancestor_record_offset":2328,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1046,"record_offset":3656,"parent_record_offset":3584,"ancestor_record_offset":2328,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1046,"record_offset":3740,"parent_record_offset":3584,"ancestor_record_offset":2328,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1046,"record_offset":4280,"parent_record_offset":3584,"ancestor_record_offset":2328,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1046,"record_offset":3420,"parent_record_offset":3392,"ancestor_record_offset":2328,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1046,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":2328,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$14::perform_layout::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1046,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":2328,"inlinee":50886,"source_line":547,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1046,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":2328,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentage::points_or_zero
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":5840,"parent_record_offset":5812,"ancestor_record_offset":2328,"inlinee":17448,"source_line":368,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Rect::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":6264,"parent_record_offset":6240,"ancestor_record_offset":2328,"inlinee":17444,"source_line":85,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Rect::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":6320,"parent_record_offset":6240,"ancestor_record_offset":2328,"inlinee":17443,"source_line":81,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Size::points_or_zero
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":6044,"parent_record_offset":6004,"ancestor_record_offset":2328,"inlinee":17451,"source_line":384,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1046,"record_offset":9236,"rva":11028992,"va":5379738112}
// 编译实例 RVAs: 0xa84a00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6988567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379738112,"raw_hex":"e8e95c95ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5379738258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6990031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379738112,"raw_hex":"e8315795ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5379739738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23239863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379738112,"raw_hex":"e8b79c6201","size":5,"soft_fail":false,"target_va":5402978144,"va":5379738276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-195237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379738112,"raw_hex":"e85b05fdff","size":5,"soft_fail":false,"target_va":5379544224,"va":5379739456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2212492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379738112,"raw_hex":"e8743ddeff","size":5,"soft_fail":false,"target_va":5377527120,"va":5379739607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1046,"record_offset":10960,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1046,"record_offset":9456,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1046,"record_offset":10848,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1046,"record_offset":10240,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":11520,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1046,"record_offset":10324,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1046,"record_offset":10512,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1046,"record_offset":10680,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1046,"record_offset":10152,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":10208,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":17449,"source_line":375,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":9876,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":17471,"source_line":409,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::size_vec2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":10068,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":17453,"source_line":116,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Size::points
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":10392,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":17469,"source_line":388,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1046,"record_offset":9736,"parent_record_offset":9712,"ancestor_record_offset":9236,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1046,"record_offset":9760,"parent_record_offset":9736,"ancestor_record_offset":9236,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1046,"record_offset":11696,"parent_record_offset":11672,"ancestor_record_offset":9236,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1046,"record_offset":9712,"parent_record_offset":9600,"ancestor_record_offset":9236,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1046,"record_offset":11672,"parent_record_offset":11572,"ancestor_record_offset":9236,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":11604,"parent_record_offset":11572,"ancestor_record_offset":9236,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":11036,"parent_record_offset":11004,"ancestor_record_offset":9236,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":9600,"parent_record_offset":9552,"ancestor_record_offset":9236,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":9552,"parent_record_offset":9504,"ancestor_record_offset":9236,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":9828,"parent_record_offset":9552,"ancestor_record_offset":9236,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":11572,"parent_record_offset":11544,"ancestor_record_offset":9236,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1046,"record_offset":11004,"parent_record_offset":10960,"ancestor_record_offset":9236,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1046,"record_offset":9504,"parent_record_offset":9456,"ancestor_record_offset":9236,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1046,"record_offset":9656,"parent_record_offset":9600,"ancestor_record_offset":9236,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1046,"record_offset":9680,"parent_record_offset":9656,"ancestor_record_offset":9236,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1046,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":9236,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1046,"record_offset":11636,"parent_record_offset":11604,"ancestor_record_offset":9236,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1046,"record_offset":10116,"parent_record_offset":10092,"ancestor_record_offset":9236,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1046,"record_offset":10176,"parent_record_offset":10152,"ancestor_record_offset":9236,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":9236,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1046,"record_offset":10360,"parent_record_offset":10324,"ancestor_record_offset":9236,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":9784,"parent_record_offset":9760,"ancestor_record_offset":9236,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":11068,"parent_record_offset":10960,"ancestor_record_offset":9236,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":9920,"parent_record_offset":9876,"ancestor_record_offset":9236,"inlinee":17449,"source_line":375,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Rect::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":10092,"parent_record_offset":10068,"ancestor_record_offset":9236,"inlinee":17443,"source_line":81,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1046,"record_offset":6648,"rva":11027216,"va":5379736336}
// 编译实例 RVAs: 0xa84310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6986791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379736336,"raw_hex":"e8d96395ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5379736482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6988303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379736336,"raw_hex":"e8f15d95ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5379738010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23241639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379736336,"raw_hex":"e8a7a36201","size":5,"soft_fail":false,"target_va":5402978144,"va":5379736500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-193509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379736336,"raw_hex":"e81b0cfdff","size":5,"soft_fail":false,"target_va":5379544224,"va":5379737728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2210764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379736336,"raw_hex":"e83444deff","size":5,"soft_fail":false,"target_va":5377527120,"va":5379737879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1046,"record_offset":8400,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1046,"record_offset":6868,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1046,"record_offset":8288,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1046,"record_offset":7680,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":8984,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1046,"record_offset":7764,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1046,"record_offset":7952,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1046,"record_offset":8120,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1046,"record_offset":7592,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":7648,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":17449,"source_line":375,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":7288,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":17471,"source_line":409,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::size_vec2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":7508,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":17453,"source_line":116,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Size::points
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":7832,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":17469,"source_line":388,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1046,"record_offset":7148,"parent_record_offset":7124,"ancestor_record_offset":6648,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1046,"record_offset":7172,"parent_record_offset":7148,"ancestor_record_offset":6648,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1046,"record_offset":9160,"parent_record_offset":9136,"ancestor_record_offset":6648,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1046,"record_offset":7124,"parent_record_offset":7012,"ancestor_record_offset":6648,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1046,"record_offset":9136,"parent_record_offset":9036,"ancestor_record_offset":6648,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":9068,"parent_record_offset":9036,"ancestor_record_offset":6648,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":8476,"parent_record_offset":8444,"ancestor_record_offset":6648,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":7012,"parent_record_offset":6964,"ancestor_record_offset":6648,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":6964,"parent_record_offset":6916,"ancestor_record_offset":6648,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":7240,"parent_record_offset":6964,"ancestor_record_offset":6648,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":9036,"parent_record_offset":9008,"ancestor_record_offset":6648,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1046,"record_offset":8444,"parent_record_offset":8400,"ancestor_record_offset":6648,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1046,"record_offset":6916,"parent_record_offset":6868,"ancestor_record_offset":6648,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1046,"record_offset":7068,"parent_record_offset":7012,"ancestor_record_offset":6648,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1046,"record_offset":7092,"parent_record_offset":7068,"ancestor_record_offset":6648,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1046,"record_offset":7976,"parent_record_offset":7952,"ancestor_record_offset":6648,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1046,"record_offset":9100,"parent_record_offset":9068,"ancestor_record_offset":6648,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1046,"record_offset":7556,"parent_record_offset":7532,"ancestor_record_offset":6648,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1046,"record_offset":7616,"parent_record_offset":7592,"ancestor_record_offset":6648,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":9008,"parent_record_offset":8984,"ancestor_record_offset":6648,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1046,"record_offset":7800,"parent_record_offset":7764,"ancestor_record_offset":6648,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":6648,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":8508,"parent_record_offset":8400,"ancestor_record_offset":6648,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":7348,"parent_record_offset":7288,"ancestor_record_offset":6648,"inlinee":17449,"source_line":375,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::Rect::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":7532,"parent_record_offset":7508,"ancestor_record_offset":6648,"inlinee":17444,"source_line":85,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1046,"record_offset":12864,"rva":11031616,"va":5379740736}
// 编译实例 RVAs: 0xa85440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t23284259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379740736,"raw_hex":"e8234a6301","size":5,"soft_fail":false,"target_va":5403025952,"va":5379741688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141604670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12055043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379740736,"raw_hex":"e803f2b700","size":5,"soft_fail":false,"target_va":5391795824,"va":5379740776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141635fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12257446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379740736,"raw_hex":"e8a608bb00","size":5,"soft_fail":false,"target_va":5391998880,"va":5379741429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142126110
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23726824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379740736,"raw_hex":"e8e80a6a01","size":5,"soft_fail":false,"target_va":5403468048,"va":5379741219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379740736,"raw_hex":"e868e5ffff","size":5,"soft_fail":false,"target_va":5379734576,"va":5379741379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379740736,"raw_hex":"e8cee5ffff","size":5,"soft_fail":false,"target_va":5379734800,"va":5379741501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":13864,"parent_record_offset":12864,"ancestor_record_offset":12864,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":13032,"parent_record_offset":12864,"ancestor_record_offset":12864,"inlinee":51266,"source_line":714,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1046,"record_offset":13404,"parent_record_offset":13380,"ancestor_record_offset":12864,"inlinee":9813,"source_line":750,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1046,"record_offset":13380,"parent_record_offset":13352,"ancestor_record_offset":12864,"inlinee":9812,"source_line":719,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1046,"record_offset":13680,"parent_record_offset":13648,"ancestor_record_offset":12864,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":13740,"parent_record_offset":13032,"ancestor_record_offset":12864,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":13648,"parent_record_offset":13032,"ancestor_record_offset":12864,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":13120,"parent_record_offset":13032,"ancestor_record_offset":12864,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":13620,"parent_record_offset":13032,"ancestor_record_offset":12864,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":13228,"parent_record_offset":13192,"ancestor_record_offset":12864,"inlinee":51269,"source_line":1665,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":13064,"parent_record_offset":13032,"ancestor_record_offset":12864,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":13192,"parent_record_offset":13156,"ancestor_record_offset":12864,"inlinee":51268,"source_line":1826,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1046,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":12864,"inlinee":9869,"source_line":1223,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":13156,"parent_record_offset":13032,"ancestor_record_offset":12864,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":13352,"parent_record_offset":13228,"ancestor_record_offset":12864,"inlinee":9811,"source_line":227,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":13552,"parent_record_offset":13228,"ancestor_record_offset":12864,"inlinee":9819,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":13580,"parent_record_offset":13228,"ancestor_record_offset":12864,"inlinee":9820,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":13496,"parent_record_offset":13228,"ancestor_record_offset":12864,"inlinee":9817,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":13324,"parent_record_offset":13228,"ancestor_record_offset":12864,"inlinee":9810,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":13524,"parent_record_offset":13228,"ancestor_record_offset":12864,"inlinee":9818,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":13468,"parent_record_offset":13228,"ancestor_record_offset":12864,"inlinee":9816,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":13296,"parent_record_offset":13228,"ancestor_record_offset":12864,"inlinee":9809,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1046,"record_offset":13436,"parent_record_offset":13352,"ancestor_record_offset":12864,"inlinee":9814,"source_line":251,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":13704,"parent_record_offset":13680,"ancestor_record_offset":12864,"inlinee":51271,"source_line":692,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":13768,"parent_record_offset":13032,"ancestor_record_offset":12864,"inlinee":51266,"source_line":714,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::impl$25::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":13924,"parent_record_offset":13900,"ancestor_record_offset":12864,"inlinee":17473,"source_line":703,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1046,"record_offset":11772,"rva":11030720,"va":5379739840}
// 编译实例 RVAs: 0xa850c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t23285405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379739840,"raw_hex":"e89d4e6301","size":5,"soft_fail":false,"target_va":5403025952,"va":5379740542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141635fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12258524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379739840,"raw_hex":"e8dc0cbb00","size":5,"soft_fail":false,"target_va":5391998880,"va":5379740351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141636ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12262070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379739840,"raw_hex":"e8b61abb00","size":5,"soft_fail":false,"target_va":5392001952,"va":5379739877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142126110
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23727878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379739840,"raw_hex":"e8060f6a01","size":5,"soft_fail":false,"target_va":5403468048,"va":5379740165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379739840,"raw_hex":"e89ee9ffff","size":5,"soft_fail":false,"target_va":5379734576,"va":5379740301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379739840,"raw_hex":"e808eaffff","size":5,"soft_fail":false,"target_va":5379734800,"va":5379740419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":12740,"parent_record_offset":11772,"ancestor_record_offset":11772,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":11988,"parent_record_offset":11772,"ancestor_record_offset":11772,"inlinee":51266,"source_line":714,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1046,"record_offset":12308,"parent_record_offset":12284,"ancestor_record_offset":11772,"inlinee":9813,"source_line":750,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1046,"record_offset":12284,"parent_record_offset":12256,"ancestor_record_offset":11772,"inlinee":9812,"source_line":719,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1046,"record_offset":12556,"parent_record_offset":12524,"ancestor_record_offset":11772,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":12616,"parent_record_offset":11988,"ancestor_record_offset":11772,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":12524,"parent_record_offset":11988,"ancestor_record_offset":11772,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":12132,"parent_record_offset":12104,"ancestor_record_offset":11772,"inlinee":51269,"source_line":1665,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":12020,"parent_record_offset":11988,"ancestor_record_offset":11772,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":11772,"inlinee":51267,"source_line":2061,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":12104,"parent_record_offset":12076,"ancestor_record_offset":11772,"inlinee":51268,"source_line":1826,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12076,"parent_record_offset":11988,"ancestor_record_offset":11772,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12256,"parent_record_offset":12132,"ancestor_record_offset":11772,"inlinee":9811,"source_line":227,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12456,"parent_record_offset":12132,"ancestor_record_offset":11772,"inlinee":9819,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12484,"parent_record_offset":12132,"ancestor_record_offset":11772,"inlinee":9820,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12400,"parent_record_offset":12132,"ancestor_record_offset":11772,"inlinee":9817,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12228,"parent_record_offset":12132,"ancestor_record_offset":11772,"inlinee":9810,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12428,"parent_record_offset":12132,"ancestor_record_offset":11772,"inlinee":9818,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12372,"parent_record_offset":12132,"ancestor_record_offset":11772,"inlinee":9816,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12200,"parent_record_offset":12132,"ancestor_record_offset":11772,"inlinee":9809,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1046,"record_offset":12340,"parent_record_offset":12256,"ancestor_record_offset":11772,"inlinee":9814,"source_line":251,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":12580,"parent_record_offset":12556,"ancestor_record_offset":11772,"inlinee":51271,"source_line":692,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":12644,"parent_record_offset":11988,"ancestor_record_offset":11772,"inlinee":51266,"source_line":714,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::impl$25::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":12796,"parent_record_offset":12772,"ancestor_record_offset":11772,"inlinee":17473,"source_line":703,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1046,"record_offset":13992,"rva":11032720,"va":5379741840}
// 编译实例 RVAs: 0xa85890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::custom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t23283534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379741840,"raw_hex":"e84e476301","size":5,"soft_fail":false,"target_va":5403025952,"va":5379742413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a850c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379741840,"raw_hex":"e89df7ffff","size":5,"soft_fail":false,"target_va":5379739840,"va":5379741982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141604670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12053915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379741840,"raw_hex":"e89bedb700","size":5,"soft_fail":false,"target_va":5391795824,"va":5379741904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141636070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12256923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379741840,"raw_hex":"e89b06bb00","size":5,"soft_fail":false,"target_va":5391999088,"va":5379742160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379741840,"raw_hex":"e86ee2ffff","size":5,"soft_fail":false,"target_va":5379734576,"va":5379742141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-7481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379741840,"raw_hex":"e8c7e2ffff","size":5,"soft_fail":false,"target_va":5379734800,"va":5379742276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":14496,"parent_record_offset":13992,"ancestor_record_offset":13992,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":14208,"parent_record_offset":13992,"ancestor_record_offset":13992,"inlinee":51266,"source_line":714,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1046,"record_offset":14536,"parent_record_offset":14496,"ancestor_record_offset":13992,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":14372,"parent_record_offset":14208,"ancestor_record_offset":13992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":14336,"parent_record_offset":14208,"ancestor_record_offset":13992,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":14300,"parent_record_offset":14208,"ancestor_record_offset":13992,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":14244,"parent_record_offset":14208,"ancestor_record_offset":13992,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1046,"record_offset":14268,"parent_record_offset":14244,"ancestor_record_offset":13992,"inlinee":9869,"source_line":1223,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$190::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":14400,"parent_record_offset":14208,"ancestor_record_offset":13992,"inlinee":11497,"source_line":1398,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::impl$26::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":14560,"parent_record_offset":14536,"ancestor_record_offset":13992,"inlinee":51273,"source_line":728,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::width
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4423
// 首条位置: {"module":320,"inlinee":17443}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1046,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":6320,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::height
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4424
// 首条位置: {"module":320,"inlinee":17444}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1046,"record_offset":6288,"parent_record_offset":6264,"ancestor_record_offset":6264,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::size_vec2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x442D
// 首条位置: {"module":1046,"inlinee":17453}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::Rect::height
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":6264,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":17444,"source_line":85,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::Rect::width
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":6320,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":17443,"source_line":81,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1046,"record_offset":6288,"parent_record_offset":6264,"ancestor_record_offset":6240,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$11::from_points
// 记录形式: inlinee_declaration; instances=1
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC83E
// 首条位置: {"module":1046,"inlinee":51262}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$13::from_points
// 记录形式: inlinee_declaration; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC840
// 首条位置: {"module":1046,"inlinee":51264}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 209, 219, 689, 696, 714
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC842
// 首条位置: {"module":1046,"inlinee":51266}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":12616,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":12524,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":13120,"parent_record_offset":13032,"ancestor_record_offset":13032,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":13620,"parent_record_offset":13032,"ancestor_record_offset":13032,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":12020,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":7227,"source_line":294,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$190::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":14400,"parent_record_offset":14208,"ancestor_record_offset":14208,"inlinee":11497,"source_line":1398,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12076,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":12644,"parent_record_offset":11988,"ancestor_record_offset":11988,"inlinee":51266,"source_line":714,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1046,"record_offset":12308,"parent_record_offset":12284,"ancestor_record_offset":11988,"inlinee":9813,"source_line":750,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1046,"record_offset":12284,"parent_record_offset":12256,"ancestor_record_offset":11988,"inlinee":9812,"source_line":719,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1046,"record_offset":12556,"parent_record_offset":12524,"ancestor_record_offset":11988,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":12132,"parent_record_offset":12104,"ancestor_record_offset":11988,"inlinee":51269,"source_line":1665,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":11988,"inlinee":51267,"source_line":2061,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":12104,"parent_record_offset":12076,"ancestor_record_offset":11988,"inlinee":51268,"source_line":1826,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1046,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":13032,"inlinee":9869,"source_line":1223,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12256,"parent_record_offset":12132,"ancestor_record_offset":11988,"inlinee":9811,"source_line":227,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i16
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12456,"parent_record_offset":12132,"ancestor_record_offset":11988,"inlinee":9819,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12484,"parent_record_offset":12132,"ancestor_record_offset":11988,"inlinee":9820,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12400,"parent_record_offset":12132,"ancestor_record_offset":11988,"inlinee":9817,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12228,"parent_record_offset":12132,"ancestor_record_offset":11988,"inlinee":9810,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12428,"parent_record_offset":12132,"ancestor_record_offset":11988,"inlinee":9818,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12372,"parent_record_offset":12132,"ancestor_record_offset":11988,"inlinee":9816,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":12200,"parent_record_offset":12132,"ancestor_record_offset":11988,"inlinee":9809,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::copysign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1046,"record_offset":12340,"parent_record_offset":12256,"ancestor_record_offset":11988,"inlinee":9814,"source_line":251,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":12580,"parent_record_offset":12556,"ancestor_record_offset":11988,"inlinee":51271,"source_line":692,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 209, 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1046,"record_offset":14612,"rva":11033616,"va":5379742736}
// 编译实例 RVAs: 0xa85c10, 0xa85d50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t23218860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379742736,"raw_hex":"e8ac4a6201","size":5,"soft_fail":false,"target_va":5402961856,"va":5379742991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1046,"record_offset":14788,"parent_record_offset":14612,"ancestor_record_offset":14612,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1046,"record_offset":14812,"parent_record_offset":14788,"ancestor_record_offset":14612,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1046,"record_offset":14836,"parent_record_offset":14812,"ancestor_record_offset":14612,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1046,"record_offset":14860,"parent_record_offset":14836,"ancestor_record_offset":14612,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 209, 219, 696
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145DF
// 首条位置: {"module":2447,"inlinee":83423}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2447,"record_offset":8292,"parent_record_offset":8264,"ancestor_record_offset":8264,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":2475,"record_offset":552,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":66110,"source_line":2039,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":2475,"record_offset":660,"parent_record_offset":552,"ancestor_record_offset":524,"inlinee":4445,"source_line":2477,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2475,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":524,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":8328,"parent_record_offset":8292,"ancestor_record_offset":8264,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":8392,"parent_record_offset":8292,"ancestor_record_offset":8264,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2475,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":524,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2475,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":524,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2475,"record_offset":708,"parent_record_offset":688,"ancestor_record_offset":524,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":8364,"parent_record_offset":8292,"ancestor_record_offset":8264,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2475,"record_offset":608,"parent_record_offset":552,"ancestor_record_offset":524,"inlinee":9828,"source_line":1383,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":2475,"record_offset":688,"parent_record_offset":552,"ancestor_record_offset":524,"inlinee":51277,"source_line":696,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":2475,"record_offset":632,"parent_record_offset":608,"ancestor_record_offset":524,"inlinee":51260,"source_line":696,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 209, 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC84E
// 首条位置: {"module":2447,"inlinee":51278}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":64116,"parent_record_offset":64064,"ancestor_record_offset":64064,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":64088,"parent_record_offset":64064,"ancestor_record_offset":64064,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$15::default
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 260
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x442F
// 首条位置: {"module":320,"record_offset":244,"rva":4472256,"va":5373181376}
// 编译实例 RVAs: 0x443dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::horizontal
// 记录形式: compiled_procedure; instances=1
// 源码行: 278
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":320,"record_offset":360,"rva":4472304,"va":5373181424}
// 编译实例 RVAs: 0x443df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LayoutStyle::horizontal
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":320,"record_offset":476,"parent_record_offset":360,"ancestor_record_offset":360,"inlinee":17446,"source_line":283,"archive":"_pdb/A/modules/00320.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::horizontal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4426
// 首条位置: {"module":320,"inlinee":17446}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::vertical
// 记录形式: inlinee_declaration; instances=1
// 源码行: 288
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4425
// 首条位置: {"module":320,"inlinee":17445}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::gap
// 记录形式: compiled_procedure; instances=1
// 源码行: 293
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":320,"record_offset":504,"rva":4472352,"va":5373181472}
// 编译实例 RVAs: 0x443e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::padding
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 298
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4433
// 首条位置: {"module":1025,"inlinee":17459}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::space_between
// 记录形式: compiled_procedure; instances=1
// 源码行: 308
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":320,"record_offset":620,"rva":4472416,"va":5373181536}
// 编译实例 RVAs: 0x443e60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::align_center
// 记录形式: compiled_procedure; instances=1
// 源码行: 318
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":320,"record_offset":744,"rva":4472464,"va":5373181584}
// 编译实例 RVAs: 0x443e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::xalign_center
// 记录形式: compiled_procedure; instances=1
// 源码行: 335
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":320,"record_offset":868,"rva":4472512,"va":5373181632}
// 编译实例 RVAs: 0x443ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::size
// 记录形式: compiled_procedure; instances=1
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":320,"record_offset":992,"rva":4472560,"va":5373181680}
// 编译实例 RVAs: 0x443ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::ComputedLayout::center
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 362
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43D1
// 首条位置: {"module":319,"inlinee":17361}
// 编译实例 RVAs: 0x443f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":319,"record_offset":69448,"parent_record_offset":69368,"ancestor_record_offset":69368,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":319,"record_offset":69392,"parent_record_offset":69368,"ancestor_record_offset":69368,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":319,"record_offset":69416,"parent_record_offset":69392,"ancestor_record_offset":69368,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":319,"record_offset":69472,"parent_record_offset":69448,"ancestor_record_offset":69368,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LengthPercentage::points_or_zero
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 368
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4428
// 首条位置: {"module":320,"inlinee":17448}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LengthPercentage::points
// 记录形式: inline_instance, inlinee_declaration; instances=31
// 源码行: 375
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4429
// 首条位置: {"module":320,"inlinee":17449}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Size::points_or_zero
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 384
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x442B
// 首条位置: {"module":320,"inlinee":17451}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points_or_zero
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":5840,"parent_record_offset":5812,"ancestor_record_offset":5812,"inlinee":17448,"source_line":368,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Size::points
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 388
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x443D
// 首条位置: {"module":1046,"inlinee":17469}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=17
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":7832,"inlinee":17449,"source_line":375,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::points_or_zero
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x443E
// 首条位置: {"module":1046,"inlinee":17470}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::Size::points_or_zero
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":6044,"parent_record_offset":6004,"ancestor_record_offset":6004,"inlinee":17451,"source_line":384,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LengthPercentage::points_or_zero
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":6072,"parent_record_offset":6044,"ancestor_record_offset":6004,"inlinee":17448,"source_line":368,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::Rect::points
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 409
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x443F
// 首条位置: {"module":1046,"inlinee":17471}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::layout::LengthPercentage::points
//     owner=private_source_ownership_candidate; count=10
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":1046,"record_offset":7348,"parent_record_offset":7288,"ancestor_record_offset":7288,"inlinee":17449,"source_line":375,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LayoutStyle::place_childen
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 458
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43C7
// 首条位置: {"module":319,"inlinee":17351}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 692
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC847
// 首条位置: {"module":1046,"inlinee":51271}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 696
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4443
// 首条位置: {"module":1046,"inlinee":17475}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 696
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1046,"record_offset":16044,"rva":11034192,"va":5379743312}
// 编译实例 RVAs: 0xa85e50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t23218383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379743312,"raw_hex":"e8cf486201","size":5,"soft_fail":false,"target_va":5402961856,"va":5379743468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6993832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379743312,"raw_hex":"e8584895ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5379743539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde::__private::string::from_utf8_lossy
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2382092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379743312,"raw_hex":"e8f4a6dbff","size":5,"soft_fail":false,"target_va":5377361344,"va":5379743431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":16224,"parent_record_offset":16044,"ancestor_record_offset":16044,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1046,"record_offset":16368,"parent_record_offset":16344,"ancestor_record_offset":16044,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1046,"record_offset":16344,"parent_record_offset":16320,"ancestor_record_offset":16044,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1046,"record_offset":16320,"parent_record_offset":16296,"ancestor_record_offset":16044,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1046,"record_offset":16248,"parent_record_offset":16224,"ancestor_record_offset":16044,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 696
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC84D
// 首条位置: {"module":2475,"inlinee":51277}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2475,"record_offset":708,"parent_record_offset":688,"ancestor_record_offset":688,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2475,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":688,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2475,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":688,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2475,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":688,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 696
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC83C
// 首条位置: {"module":1046,"inlinee":51260}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 696
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1046,"record_offset":16620,"rva":11034512,"va":5379743632}
// 编译实例 RVAs: 0xa85f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t23216831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379743632,"raw_hex":"e8bf426201","size":5,"soft_fail":false,"target_va":5402960976,"va":5379744140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141635df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12254795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379743632,"raw_hex":"e84bfeba00","size":5,"soft_fail":false,"target_va":5391998448,"va":5379743648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142126110
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: layout.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23723330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379743632,"raw_hex":"e842fd6901","size":5,"soft_fail":false,"target_va":5403468048,"va":5379744713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":18052,"parent_record_offset":16620,"ancestor_record_offset":16620,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":16896,"parent_record_offset":16620,"ancestor_record_offset":16620,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1046,"record_offset":17504,"parent_record_offset":16620,"ancestor_record_offset":16620,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1046,"record_offset":16868,"parent_record_offset":16620,"ancestor_record_offset":16620,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1046,"record_offset":16932,"parent_record_offset":16620,"ancestor_record_offset":16620,"inlinee":10107,"source_line":2117,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1046,"record_offset":17304,"parent_record_offset":17280,"ancestor_record_offset":16620,"inlinee":9813,"source_line":750,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1046,"record_offset":17280,"parent_record_offset":17252,"ancestor_record_offset":16620,"inlinee":9812,"source_line":719,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":17184,"parent_record_offset":17148,"ancestor_record_offset":16620,"inlinee":51269,"source_line":1665,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":17148,"parent_record_offset":17112,"ancestor_record_offset":16620,"inlinee":51268,"source_line":1826,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$26::newtype_variant_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1046,"record_offset":16960,"parent_record_offset":16932,"ancestor_record_offset":16620,"inlinee":25269,"source_line":2142,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1046,"record_offset":17076,"parent_record_offset":17032,"ancestor_record_offset":16620,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":17112,"parent_record_offset":17076,"ancestor_record_offset":16620,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":17252,"parent_record_offset":17184,"ancestor_record_offset":16620,"inlinee":9811,"source_line":227,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":17452,"parent_record_offset":17184,"ancestor_record_offset":16620,"inlinee":9820,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":17424,"parent_record_offset":17184,"ancestor_record_offset":16620,"inlinee":9817,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":17368,"parent_record_offset":17184,"ancestor_record_offset":16620,"inlinee":9815,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1046,"record_offset":17396,"parent_record_offset":17184,"ancestor_record_offset":16620,"inlinee":9816,"source_line":193,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::copysign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1046,"record_offset":17336,"parent_record_offset":17252,"ancestor_record_offset":16620,"inlinee":9814,"source_line":251,"archive":"_pdb/A/modules/01046.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$25::from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 703
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4441
// 首条位置: {"module":1046,"inlinee":17473}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::layout::impl$26::from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 728
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC849
// 首条位置: {"module":1046,"inlinee":51273}
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
// 完整结构及原始类型记录: layout.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::Align
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<toolbar::mei::layout::Align>", "constructor": "core::marker::PhantomData", "arguments": ["toolbar::mei::layout::Align"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Align> >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<toolbar::mei::layout::Align>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::ComputedLayout
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<toolbar::mei::layout::ComputedLayout,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["toolbar::mei::layout::ComputedLayout", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::LayoutStyle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 17; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >", "constructor": "toolbar::mei::layout::LayoutStyle::compute_min_size_with_children", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>", "constructor": "toolbar::mei::layout::LayoutStyle::place_childen_impl", "arguments": ["toolbar::mei::layout::SwizzleYX"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>", "constructor": "toolbar::mei::layout::LayoutStyle::place_childen_impl", "arguments": ["toolbar::mei::layout::SwizzleXY"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::LayoutTemp
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<toolbar::mei::layout::LayoutTemp,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["toolbar::mei::layout::LayoutTemp", "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$1> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["toolbar::mei::layout::LayoutTemp", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$19::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$1> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["toolbar::mei::layout::LayoutTemp", "alloc::vec::impl$19::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<toolbar::mei::core::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$14::perform_layout::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$19::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$1> > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["toolbar::mei::core::GlobalId", "toolbar::mei::layout::LayoutTemp", "tuple$<>", "toolbar::mei::core::impl$14::perform_layout::closure_env$1", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$19::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<toolbar::mei::core::GlobalId>,toolbar::mei::core::GlobalId,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<toolbar::mei::core::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$14::perform_layout::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$19::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$1> > > > >", "constructor": "core::iter::adapters::copied::impl$1::fold", "arguments": ["core::slice::iter::Iter<toolbar::mei::core::GlobalId>", "toolbar::mei::core::GlobalId", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<toolbar::mei::core::GlobalId,toolbar::mei::layout::LayoutTemp,tuple$<>,toolbar::mei::core::impl$14::perform_layout::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::vec::impl$19::extend_trusted::closure_env$0<toolbar::mei::layout::LayoutTemp,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$1> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::LengthPercentage
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "enum2$<toolbar::mei::layout::LengthPercentage>", "constructor": "enum2$", "arguments": ["toolbar::mei::layout::LengthPercentage"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >", "constructor": "toolbar::mei::layout::_::impl$0::deserialize", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>", "ref_mut$<ron::de::Deserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "toolbar::mei::layout::Rect", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "core::marker::PhantomData", "arguments": ["toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > > >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "core::marker::PhantomData", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::SeqAccess::next_element<ron::de::CommaSeparated,enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "serde::de::SeqAccess::next_element", "arguments": ["ron::de::CommaSeparated", "enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "serde::de::impls::impl$190::deserialize::TupleVisitor", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>", "enum2$<toolbar::mei::layout::LengthPercentage>", "enum2$<toolbar::mei::layout::LengthPercentage>", "enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::__private::de::content::impl$21::deserialize_tuple<enum2$<ron::error::Error>,serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "serde::__private::de::content::impl$21::deserialize_tuple", "arguments": ["enum2$<ron::error::Error>", "serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$1::fmt<serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "serde::de::impl$1::fmt", "arguments": ["serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::LengthPercentageDeser::Naked
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::Rect
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> >", "constructor": "toolbar::mei::layout::Rect", "arguments": ["enum2$<toolbar::mei::layout::LengthPercentage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > >", "constructor": "core::marker::PhantomData", "arguments": ["toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > > >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::Size
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::SwizzleXY
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>", "constructor": "toolbar::mei::layout::LayoutStyle::place_childen_impl", "arguments": ["toolbar::mei::layout::SwizzleXY"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::SwizzleYX
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>", "constructor": "toolbar::mei::layout::LayoutStyle::place_childen_impl", "arguments": ["toolbar::mei::layout::SwizzleYX"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::layout::XAlign
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<toolbar::mei::layout::XAlign>", "constructor": "core::marker::PhantomData", "arguments": ["toolbar::mei::layout::XAlign"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::XAlign> >", "constructor": "ron::de::impl$9::newtype_variant_seed", "arguments": ["core::marker::PhantomData<toolbar::mei::layout::XAlign>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: layout.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: layout.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::deferred_wall_history::PendingWallHistory>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::deferred_wall_history::PendingWallHistory>", "bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::deferred_wall_history::PendingWallHistory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::deferred_wall_history::PendingWallHistory>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::deferred_wall_history::PendingWallHistory>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::deferred_wall_history::PendingWallHistory>

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::ComputedLayout::center
// 信息 ID: 2e54c8e9480cc4792b900010
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x33f7b6 -> 0x443f30 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::core::mei_post_update; module=319, inline_record=69368; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::align_center
// 信息 ID: c594085eb3e3206836612aa6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$18::pill_button<ref$<str$> > @ 0x7f9fa -> 0x443e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >
// 信息 ID: 7494dd52634ed7c47a5a4155
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["toolbar::mei::core::GlobalId"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<toolbar::mei::core::GlobalId>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >", "toolbar::mei::core::impl$14::perform_layout::closure_env$0"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children -> ["core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x4414b7 -> 0xa83f00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::gap
// 信息 ID: 407e87c9d53b942b15cfca34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x6a2aa -> 0x443e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x6a372 -> 0x443e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x6a439 -> 0x443e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x6a504 -> 0x443e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::horizontal
// 信息 ID: 52c17fdc75043ff88a3885d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::horizontal; module=320, inline_record=476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::padding
// 信息 ID: 7e9b4046908e4c3233b1709a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=1025, inline_record=8496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::place_childen
// 信息 ID: 526dec28122566809cc2d802
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::mei_post_update; module=319, inline_record=60540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>
// 信息 ID: c3dd6d265b820c05c47895cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::layout::LayoutStyle::place_childen_impl -> ["toolbar::mei::layout::SwizzleXY"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x441d6c -> 0xa84a00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>
// 信息 ID: 6b38bc7e48b4c120f8161ea5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::layout::LayoutStyle::place_childen_impl -> ["toolbar::mei::layout::SwizzleYX"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x441d50 -> 0xa84310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::size
// 信息 ID: 8f1656e7aff718dff7862f5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$18::pill_button<ref$<str$> > @ 0x7fa1f -> 0x443ef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::space_between
// 信息 ID: 918b018a4f881858b8e475b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d927 -> 0x443e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1df0c -> 0x443e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e4fc -> 0x443e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eb3c -> 0x443e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f0cc -> 0x443e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f659 -> 0x443e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1faff -> 0x443e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x200d9 -> 0x443e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > > @ 0x205bc -> 0x443e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x20b84 -> 0x443e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> > @ 0x2102f -> 0x443e60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::vertical
// 信息 ID: e9a9bbcec27c9c1786e5ebea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LayoutStyle::xalign_center
// 信息 ID: 000b4209fb0c0722371f6a83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$18::pill_button<ref$<str$> > @ 0x7fa05 -> 0x443ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LengthPercentage::points
// 信息 ID: 71d6575ea78448b75b144511
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 29 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7348; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7376; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7408; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7440; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7472; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7648; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7856; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7916; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LengthPercentage::points_or_zero
// 信息 ID: 965dcb012cf9e07f3f3c5d9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=2700; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=5840; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=5872; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=5936; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=5968; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=6072; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=6104; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=6168; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::LengthPercentageDeser::Naked::constructor$0
// 信息 ID: 1783c54ebb01bad5fe2a5a19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=1046, inline_record=12580; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=1046, inline_record=13704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::height
// 信息 ID: e4aa50355af8bd52c6db0fb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=6264; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::points
// 信息 ID: 411b33d1b192a50ed42f6757
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7288; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1046, inline_record=9876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::points_or_zero
// 信息 ID: 39aaeb3eb1b6d4b346ed4fbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=6004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::size_vec2
// 信息 ID: 7a37607c6641a84a0fafd775
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=6240; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7508; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1046, inline_record=10068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Rect::width
// 信息 ID: 5a24c33af544025d209be4fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=6320; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1046, inline_record=10092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Size::points
// 信息 ID: 9785984a924aab3331248d64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7832; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=7888; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=8064; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=8540; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=8632; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleYX>; module=1046, inline_record=8760; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1046, inline_record=10392; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::place_childen_impl<toolbar::mei::layout::SwizzleXY>; module=1046, inline_record=10448; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::Size::points_or_zero
// 信息 ID: 4d224c0ad966433edf13b9aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=5812; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=5908; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=6044; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::LayoutStyle::compute_min_size_with_children<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<toolbar::mei::core::GlobalId> >,toolbar::mei::core::impl$14::perform_layout::closure_env$0> >; module=1046, inline_record=6140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize
// 信息 ID: c6092402b661a5ff902ff0a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=1046, inline_record=11988; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=1046, inline_record=12644; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=1046, inline_record=13032; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=1046, inline_record=13768; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >; module=1046, inline_record=14208; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Align> >; module=2447, inline_record=20696; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::XAlign> >; module=2447, inline_record=22984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 6899df58950223853efe92bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<toolbar::mei::layout::_::impl$0::deserialize::__FieldVisitor>; module=1046, inline_record=412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 信息 ID: 37cb60714972a5478cef8920
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_bytes -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 48a52ee5bac6e64a45abee94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2475, inline_record=688; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2475, inline_record=880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: cd696b7f41ba6daebe69add7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> > @ 0x15fee17 -> 0xa85c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> > @ 0x15fddc7 -> 0xa85d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: 566a431eb4678aab01741f51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2475, inline_record=632; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2475, inline_record=824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 59500c1fd31f03a58dea8bdc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2447, inline_record=8264; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2447, inline_record=13220; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<toolbar::mei::layout::_::impl$0::deserialize::__Field> >; module=2475, inline_record=524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 23b1201bc28bfd08513f61e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<toolbar::mei::layout::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=64064; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<toolbar::mei::layout::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=65252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> > >
// 信息 ID: 9f1a87fcd62e56e3356f71c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::__private::de::content::EnumRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize::impl$2::visit_enum -> ["serde::__private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_enum<enum2$<ron::error::Error>,toolbar::mei::layout::_::impl$0::deserialize::__Visitor> @ 0x1635fd6 -> 0xa85f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >
// 信息 ID: 13533c72973355e6064bda50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["toolbar::mei::layout::LengthPercentage"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::de::Deserializer"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize -> ["enum2$<toolbar::mei::layout::LengthPercentage>", "ref_mut$<ron::de::Deserializer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<toolbar::mei::layout::Rect<enum2$<toolbar::mei::layout::LengthPercentage> > > > @ 0x1601076 -> 0xa85890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >
// 信息 ID: f245a5a924640c8fb68ba4b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::Deserializer"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize -> ["ref_mut$<ron::de::Deserializer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x1601adf -> 0xa85440 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::SeqAccess::next_element<ron::de::CommaSeparated,enum2$<toolbar::mei::layout::LengthPercentage> > @ 0x1601e22 -> 0xa85440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >
// 信息 ID: 5feeed8b8350e3876e636a57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::__private::de::content::ContentRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::layout::_::impl$0::deserialize -> ["serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> > @ 0xa8591e -> 0xa850c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_tuple<enum2$<ron::error::Error>,serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x16360ca -> 0xa850c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_tuple<enum2$<ron::error::Error>,serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x1636159 -> 0xa850c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_tuple<enum2$<ron::error::Error>,serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x163626e -> 0xa850c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_tuple<enum2$<ron::error::Error>,serde::de::impls::impl$190::deserialize::TupleVisitor<enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage>,enum2$<toolbar::mei::layout::LengthPercentage> > > @ 0x163637d -> 0xa850c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::horizontal
// 信息 ID: eea9e41e1f4833e37bd8584e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 26 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d918 -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1da44 -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1defd -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e02f -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e4ed -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e61f -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eb2d -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ec62 -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f0bd -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f1f2 -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f64a -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f77e -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1faf0 -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fc22 -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x200ca -> 0x443df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x201f7 -> 0x443df0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$11::from_points
// 信息 ID: ca1a0759ab3630171bcc7ab7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$13::from_points
// 信息 ID: e875dd4797b1f39bad19543d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$15::default
// 信息 ID: 9b7439a9d8622a172a60a397
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$18::pill_button<ref$<str$> > @ 0x7f9eb -> 0x443dc0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=1025, inline_record=1996; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$25::from
// 信息 ID: 727a8d460877693ae0d777a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=1046, inline_record=12796; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >; module=1046, inline_record=13924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::layout::impl$26::from
// 信息 ID: f9f5c18d5e91d066fb8ec5ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::layout::_::impl$0::deserialize<enum2$<toolbar::mei::layout::LengthPercentage>,ref_mut$<ron::de::Deserializer> >; module=1046, inline_record=14560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
