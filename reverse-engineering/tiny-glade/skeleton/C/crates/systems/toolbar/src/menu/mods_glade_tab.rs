// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/menu/mods_glade_tab.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: dd563e758ef8334b756ec9c850f4722506d0148443f0703fe1082ebba2a06981
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mods_glade_tab.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2896
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_state; // namespace candidate
//   package identities: [{"name":"bevy_state","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_glade_tab::list_slot
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2896,"record_offset":10432,"rva":37009776,"va":5405718896}
// 编译实例 RVAs: 0x234b970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32240039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e8590e14fe","size":5,"soft_fail":false,"target_va":5373479040,"va":5405719074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32234607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e8912314fe","size":5,"soft_fail":false,"target_va":5373484384,"va":5405718986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32234227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e80d2514fe","size":5,"soft_fail":false,"target_va":5373484784,"va":5405719006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24654786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e83ecc87fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5405718957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18695991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e8c9b8e2fe","size":5,"soft_fail":false,"target_va":5387023552,"va":5405719538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18694040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e868c0e2fe","size":5,"soft_fail":false,"target_va":5387025456,"va":5405719491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117c4b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18675891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e84d07e3fe","size":5,"soft_fail":false,"target_va":5387044016,"va":5405719902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18514871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e8497ce5fe","size":5,"soft_fail":false,"target_va":5387204688,"va":5405719554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18513798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e87a80e5fe","size":5,"soft_fail":false,"target_va":5387205712,"va":5405719505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14231a7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-202238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e802eafcff","size":5,"soft_fail":false,"target_va":5405517728,"va":5405719961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10901601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e86158a600","size":5,"soft_fail":false,"target_va":5416621600,"va":5405719994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22584649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e8b762a7fe","size":5,"soft_fail":false,"target_va":5383134720,"va":5405719364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29510659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405718896,"raw_hex":"e8fdb33dfe","size":5,"soft_fail":false,"target_va":5376208368,"va":5405719022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2896,"record_offset":13168,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2896,"record_offset":10584,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2896,"record_offset":10724,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2896,"record_offset":12032,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2896,"record_offset":11972,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2896,"record_offset":11768,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2896,"record_offset":12092,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::mod_checklist::replacement_mod_checklist
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/mod_checklist.rs
//     evidence={"module":2896,"record_offset":13224,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":111818,"source_line":66,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2896,"record_offset":13256,"parent_record_offset":13224,"ancestor_record_offset":10432,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2896,"record_offset":11496,"parent_record_offset":11472,"ancestor_record_offset":10432,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2896,"record_offset":11064,"parent_record_offset":11040,"ancestor_record_offset":10432,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2896,"record_offset":11148,"parent_record_offset":11124,"ancestor_record_offset":10432,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2896,"record_offset":11564,"parent_record_offset":10928,"ancestor_record_offset":10432,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2896,"record_offset":11296,"parent_record_offset":11268,"ancestor_record_offset":10432,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2896,"record_offset":11268,"parent_record_offset":11240,"ancestor_record_offset":10432,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2896,"record_offset":11736,"parent_record_offset":10724,"ancestor_record_offset":10432,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2896,"record_offset":11088,"parent_record_offset":11064,"ancestor_record_offset":10432,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2896,"record_offset":11412,"parent_record_offset":11388,"ancestor_record_offset":10432,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2896,"record_offset":10764,"parent_record_offset":10724,"ancestor_record_offset":10432,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2896,"record_offset":11520,"parent_record_offset":11496,"ancestor_record_offset":10432,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2896,"record_offset":11208,"parent_record_offset":11180,"ancestor_record_offset":10432,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2896,"record_offset":11240,"parent_record_offset":11208,"ancestor_record_offset":10432,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2896,"record_offset":11180,"parent_record_offset":10928,"ancestor_record_offset":10432,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2896,"record_offset":11040,"parent_record_offset":10928,"ancestor_record_offset":10432,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2896,"record_offset":11592,"parent_record_offset":10928,"ancestor_record_offset":10432,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2896,"record_offset":11124,"parent_record_offset":10928,"ancestor_record_offset":10432,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2896,"record_offset":10984,"parent_record_offset":10928,"ancestor_record_offset":10432,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2896,"record_offset":11472,"parent_record_offset":11448,"ancestor_record_offset":10432,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2896,"record_offset":10820,"parent_record_offset":10792,"ancestor_record_offset":10432,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2896,"record_offset":11448,"parent_record_offset":11340,"ancestor_record_offset":10432,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2896,"record_offset":10852,"parent_record_offset":10820,"ancestor_record_offset":10432,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":11388,"parent_record_offset":11364,"ancestor_record_offset":10432,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":11684,"parent_record_offset":10928,"ancestor_record_offset":10432,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":11364,"parent_record_offset":11340,"ancestor_record_offset":10432,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":10904,"parent_record_offset":10880,"ancestor_record_offset":10432,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":10880,"parent_record_offset":10820,"ancestor_record_offset":10432,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":10928,"parent_record_offset":10904,"ancestor_record_offset":10432,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":11012,"parent_record_offset":10928,"ancestor_record_offset":10432,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":11340,"parent_record_offset":10928,"ancestor_record_offset":10432,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2896,"record_offset":10792,"parent_record_offset":10724,"ancestor_record_offset":10432,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2896,"record_offset":11880,"parent_record_offset":11820,"ancestor_record_offset":10432,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2896,"record_offset":11820,"parent_record_offset":11768,"ancestor_record_offset":10432,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2896,"record_offset":12124,"parent_record_offset":12092,"ancestor_record_offset":10432,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2896,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":10432,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2896,"record_offset":10656,"parent_record_offset":10632,"ancestor_record_offset":10432,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2896,"record_offset":10632,"parent_record_offset":10608,"ancestor_record_offset":10432,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2896,"record_offset":10688,"parent_record_offset":10608,"ancestor_record_offset":10432,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2896,"record_offset":10608,"parent_record_offset":10584,"ancestor_record_offset":10432,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2896,"record_offset":12000,"parent_record_offset":11972,"ancestor_record_offset":10432,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2896,"record_offset":11936,"parent_record_offset":11820,"ancestor_record_offset":10432,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::mods_glade_tab::apply_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2896,"record_offset":1540,"rva":37005680,"va":5405714800}
// 编译实例 RVAs: 0x234a970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-20919682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e87ecac0fe","size":5,"soft_fail":false,"target_va":5384795984,"va":5405715661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21060328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e818a5befe","size":5,"soft_fail":false,"target_va":5384656240,"va":5405716563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <str>::to_uppercase
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20924865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e83fb6c0fe","size":5,"soft_fail":false,"target_va":5384790144,"va":5405715004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21107470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8f2ecbdfe","size":5,"soft_fail":false,"target_va":5384609408,"va":5405716873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30036740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8fcac35fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5405715423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-30036373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e86bae35fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5405715072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36993211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e84587cbfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5405715398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=3
//     target_aliases=26 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36286784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8c04ed6fd","size":5,"soft_fail":false,"target_va":5369431360,"va":5405718139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2470
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36277953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e83f71d6fd","size":5,"soft_fail":false,"target_va":5369439344,"va":5405717292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32236077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8d31d14fe","size":5,"soft_fail":false,"target_va":5373479040,"va":5405715112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29478182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8da323efe","size":5,"soft_fail":false,"target_va":5376236864,"va":5405715041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24110012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8441c90fe","size":5,"soft_fail":false,"target_va":5381607664,"va":5405717671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141189950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18618782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e862e6e3fe","size":5,"soft_fail":false,"target_va":5387098448,"va":5405717225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118a030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18618369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8ffe7e3fe","size":5,"soft_fail":false,"target_va":5387100208,"va":5405718572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411c0790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18392546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e81e5ae7fe","size":5,"soft_fail":false,"target_va":5387323280,"va":5405715821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10633337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e87940a200","size":5,"soft_fail":false,"target_va":5416351056,"va":5405717714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mods_glade_tab.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11342085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e80511ad00","size":5,"soft_fail":false,"target_va":5417057040,"va":5405714950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10905466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e87a67a600","size":5,"soft_fail":false,"target_va":5416620794,"va":5405715323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-21092811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e83526befe","size":5,"soft_fail":false,"target_va":5384624080,"va":5405716886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18404746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8762ae7fe","size":5,"soft_fail":false,"target_va":5387312896,"va":5405717637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10906178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8426aa600","size":5,"soft_fail":false,"target_va":5416621536,"va":5405715353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10903964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e89c61a600","size":5,"soft_fail":false,"target_va":5416621762,"va":5405717793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::session::NextSessionKind> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18603512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e80822e4fe","size":5,"soft_fail":false,"target_va":5387114928,"va":5405718435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::glade_settings::GladeSettings::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25455314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e82e957bfe","size":5,"soft_fail":false,"target_va":5380261184,"va":5405716493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25449203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e80dad7bfe","size":5,"soft_fail":false,"target_va":5380267312,"va":5405716510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::assets_root
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25454366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8e2987bfe","size":5,"soft_fail":false,"target_va":5380262112,"va":5405716473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::write_to_disk
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25454716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e884977bfe","size":5,"soft_fail":false,"target_va":5380262240,"va":5405716951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_game::parse_saved_game_data
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25335288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8086a7dfe","size":5,"soft_fail":false,"target_va":5380381344,"va":5405716627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22580270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8d273a7fe","size":5,"soft_fail":false,"target_va":5383134720,"va":5405714985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::canonical_mod_entries
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22796374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e8aa27a4fe","size":5,"soft_fail":false,"target_va":5382918496,"va":5405714865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::text_hint_popup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29534612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405714800,"raw_hex":"e86c563dfe","size":5,"soft_fail":false,"target_va":5376182752,"va":5405717359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2896,"record_offset":5124,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::partial_eq::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":2896,"record_offset":1716,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":111817,"source_line":15,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2896,"record_offset":8992,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::NextState::set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_state","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_state-0.17.3/src/state/resources.rs
//     evidence={"module":2896,"record_offset":8936,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":10613,"source_line":134,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2896,"record_offset":7024,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2896,"record_offset":2012,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2896,"record_offset":3464,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":8466,"source_line":176,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$35::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2896,"record_offset":8964,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":70225,"source_line":325,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2896,"record_offset":7080,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2896,"record_offset":6940,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":2896,"record_offset":6544,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2896,"record_offset":3380,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2896,"record_offset":2296,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2896,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":1540,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2896,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":1540,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2896,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":1540,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2896,"record_offset":2212,"parent_record_offset":2188,"ancestor_record_offset":1540,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2896,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":1540,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2896,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":1540,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2896,"record_offset":3972,"parent_record_offset":3812,"ancestor_record_offset":1540,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2896,"record_offset":2164,"parent_record_offset":2108,"ancestor_record_offset":1540,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2896,"record_offset":7556,"parent_record_offset":7528,"ancestor_record_offset":1540,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2896,"record_offset":9216,"parent_record_offset":9184,"ancestor_record_offset":1540,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2896,"record_offset":3732,"parent_record_offset":3696,"ancestor_record_offset":1540,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2896,"record_offset":2136,"parent_record_offset":2108,"ancestor_record_offset":1540,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2896,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1540,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2896,"record_offset":9264,"parent_record_offset":9240,"ancestor_record_offset":1540,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2896,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":1540,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2896,"record_offset":3812,"parent_record_offset":3768,"ancestor_record_offset":1540,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2896,"record_offset":3768,"parent_record_offset":3732,"ancestor_record_offset":1540,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2896,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1540,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2896,"record_offset":3860,"parent_record_offset":3812,"ancestor_record_offset":1540,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2896,"record_offset":5160,"parent_record_offset":5124,"ancestor_record_offset":1540,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2896,"record_offset":3636,"parent_record_offset":3564,"ancestor_record_offset":1540,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2896,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":1540,"inlinee":10830,"source_line":417,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2896,"record_offset":1860,"parent_record_offset":1828,"ancestor_record_offset":1540,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2896,"record_offset":9184,"parent_record_offset":9144,"ancestor_record_offset":1540,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2896,"record_offset":3608,"parent_record_offset":3564,"ancestor_record_offset":1540,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2896,"record_offset":9116,"parent_record_offset":9080,"ancestor_record_offset":1540,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2896,"record_offset":9144,"parent_record_offset":9116,"ancestor_record_offset":1540,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2896,"record_offset":5812,"parent_record_offset":5788,"ancestor_record_offset":1540,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2896,"record_offset":3696,"parent_record_offset":3636,"ancestor_record_offset":1540,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2896,"record_offset":3528,"parent_record_offset":3464,"ancestor_record_offset":1540,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2896,"record_offset":5884,"parent_record_offset":5860,"ancestor_record_offset":1540,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2896,"record_offset":3564,"parent_record_offset":3528,"ancestor_record_offset":1540,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":2896,"record_offset":1892,"parent_record_offset":1860,"ancestor_record_offset":1540,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2896,"record_offset":9052,"parent_record_offset":9016,"ancestor_record_offset":1540,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2896,"record_offset":9016,"parent_record_offset":8992,"ancestor_record_offset":1540,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2896,"record_offset":9080,"parent_record_offset":9016,"ancestor_record_offset":1540,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":2896,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":1540,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2896,"record_offset":1796,"parent_record_offset":1768,"ancestor_record_offset":1540,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2896,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":1540,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2896,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":1540,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2896,"record_offset":2780,"parent_record_offset":2756,"ancestor_record_offset":1540,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2896,"record_offset":3196,"parent_record_offset":2560,"ancestor_record_offset":1540,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2896,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":1540,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2896,"record_offset":4144,"parent_record_offset":4100,"ancestor_record_offset":1540,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":2896,"record_offset":4100,"parent_record_offset":3636,"ancestor_record_offset":1540,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":2896,"record_offset":4236,"parent_record_offset":3636,"ancestor_record_offset":1540,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2896,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":1540,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2896,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":1540,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2896,"record_offset":2928,"parent_record_offset":2900,"ancestor_record_offset":1540,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2896,"record_offset":2900,"parent_record_offset":2872,"ancestor_record_offset":1540,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2896,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":1540,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2896,"record_offset":4280,"parent_record_offset":3464,"ancestor_record_offset":1540,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2896,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":1540,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2896,"record_offset":2036,"parent_record_offset":2012,"ancestor_record_offset":1540,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2896,"record_offset":5552,"parent_record_offset":5528,"ancestor_record_offset":1540,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2896,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":1540,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2896,"record_offset":2352,"parent_record_offset":2324,"ancestor_record_offset":1540,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2896,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":1540,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2896,"record_offset":9304,"parent_record_offset":9144,"ancestor_record_offset":1540,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2896,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":1540,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2896,"record_offset":1768,"parent_record_offset":1740,"ancestor_record_offset":1540,"inlinee":6223,"source_line":122,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2896,"record_offset":1952,"parent_record_offset":1920,"ancestor_record_offset":1540,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2896,"record_offset":5504,"parent_record_offset":5196,"ancestor_record_offset":1540,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2896,"record_offset":5528,"parent_record_offset":5504,"ancestor_record_offset":1540,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2896,"record_offset":4172,"parent_record_offset":4144,"ancestor_record_offset":1540,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2896,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":1540,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2896,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":1540,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":2896,"record_offset":4344,"parent_record_offset":4308,"ancestor_record_offset":1540,"inlinee":8473,"source_line":24,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":2896,"record_offset":4308,"parent_record_offset":4280,"ancestor_record_offset":1540,"inlinee":8471,"source_line":27,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":2896,"record_offset":4388,"parent_record_offset":4308,"ancestor_record_offset":1540,"inlinee":8475,"source_line":11,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2896,"record_offset":4488,"parent_record_offset":4456,"ancestor_record_offset":1540,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":2896,"record_offset":4456,"parent_record_offset":4424,"ancestor_record_offset":1540,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2896,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":1540,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2896,"record_offset":2840,"parent_record_offset":2812,"ancestor_record_offset":1540,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2896,"record_offset":2872,"parent_record_offset":2840,"ancestor_record_offset":1540,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2896,"record_offset":2812,"parent_record_offset":2560,"ancestor_record_offset":1540,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2896,"record_offset":2672,"parent_record_offset":2560,"ancestor_record_offset":1540,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2896,"record_offset":3224,"parent_record_offset":2560,"ancestor_record_offset":1540,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2896,"record_offset":2756,"parent_record_offset":2560,"ancestor_record_offset":1540,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2896,"record_offset":2616,"parent_record_offset":2560,"ancestor_record_offset":1540,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2896,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":1540,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2896,"record_offset":2452,"parent_record_offset":2412,"ancestor_record_offset":1540,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2896,"record_offset":3080,"parent_record_offset":2972,"ancestor_record_offset":1540,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2896,"record_offset":2484,"parent_record_offset":2452,"ancestor_record_offset":1540,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":1540,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":3316,"parent_record_offset":2560,"ancestor_record_offset":1540,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":1540,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":1540,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":2512,"parent_record_offset":2452,"ancestor_record_offset":1540,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":2560,"parent_record_offset":2536,"ancestor_record_offset":1540,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":2644,"parent_record_offset":2560,"ancestor_record_offset":1540,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2896,"record_offset":2972,"parent_record_offset":2560,"ancestor_record_offset":1540,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2896,"record_offset":7128,"parent_record_offset":7104,"ancestor_record_offset":1540,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2896,"record_offset":7104,"parent_record_offset":7080,"ancestor_record_offset":1540,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2896,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":1540,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2896,"record_offset":2412,"parent_record_offset":2380,"ancestor_record_offset":1540,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2896,"record_offset":2380,"parent_record_offset":2324,"ancestor_record_offset":1540,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::impl$13::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":2896,"record_offset":5708,"parent_record_offset":5196,"ancestor_record_offset":1540,"inlinee":10832,"source_line":98,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":2896,"record_offset":1828,"parent_record_offset":1796,"ancestor_record_offset":1540,"inlinee":22085,"source_line":98,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2896,"record_offset":2324,"parent_record_offset":2296,"ancestor_record_offset":1540,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2896,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":1540,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/02896.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2896,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":1540,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/02896.jsonl.gz"}
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
// 完整结构及原始类型记录: mods_glade_tab.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mods_glade_tab.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mods_glade_tab.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_glade_tab::apply_button
// 信息 ID: 1dc37ddcdcaa63f08d1618d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::mods::ui::closure$5 @ 0x11a27a4 -> 0x234a970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::mods_glade_tab::list_slot
// 信息 ID: 0d7695ff8656c39d95e2bb26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::mods::ui::closure$4 @ 0x11a28d3 -> 0x234b970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
