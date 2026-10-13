// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/color/src/set_wall_color_id.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d31118bc10d9742220663b5072939cac913d6a107308338bee81cc78450c9c21
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: set_wall_color_id.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4017
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::set_wall_color_id::init_color_ids_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4017,"record_offset":3020,"rva":42643600,"va":5411352720}
// 编译实例 RVAs: 0x28ab090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-35674141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8e3a7dffd","size":5,"soft_fail":false,"target_va":5375678688,"va":5411352824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-35674184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8b8a7dffd","size":5,"soft_fail":false,"target_va":5375678704,"va":5411352883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=7
//     target_aliases=1699 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42630622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8228275fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5411352809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31636520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8d8431dfe","size":5,"soft_fail":false,"target_va":5379716240,"va":5411352755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ba15e0
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30448620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e814642ffe","size":5,"soft_fail":false,"target_va":5380904416,"va":5411353031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410a28a0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25201932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8f4727ffe","size":5,"soft_fail":false,"target_va":5386152096,"va":5411354023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c1610
//     owner=unresolved_namespace; count=5
//     target_aliases=6 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25074361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8476581fe","size":5,"soft_fail":false,"target_va":5386278416,"va":5411352772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c1760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25074825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8776381fe","size":5,"soft_fail":false,"target_va":5386278752,"va":5411353572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c18b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25074655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8216481fe","size":5,"soft_fail":false,"target_va":5386279088,"va":5411353738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142182840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7506441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8f7758dff","size":5,"soft_fail":false,"target_va":5403846720,"va":5411353156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421829e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7505723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8c5788dff","size":5,"soft_fail":false,"target_va":5403847136,"va":5411352854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142182b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7506107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e845778dff","size":5,"soft_fail":false,"target_va":5403847552,"va":5411353654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142182d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7505027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e87d7b8dff","size":5,"soft_fail":false,"target_va":5403847968,"va":5411352990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142182ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7504943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8d17b8dff","size":5,"soft_fail":false,"target_va":5403848384,"va":5411353322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142183060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7505025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e87f7b8dff","size":5,"soft_fail":false,"target_va":5403848800,"va":5411353820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142183200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7504277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e86b7e8dff","size":5,"soft_fail":false,"target_va":5403849216,"va":5411353488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428b7870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t50121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8c9c30000","size":5,"soft_fail":false,"target_va":5411403888,"va":5411353762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428b7a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t51415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8d7c80000","size":5,"soft_fail":false,"target_va":5411404352,"va":5411352932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428b7c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t51215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e80fc80000","size":5,"soft_fail":false,"target_va":5411404816,"va":5411353596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428b7de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t52177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8d1cb0000","size":5,"soft_fail":false,"target_va":5411405280,"va":5411353098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428b7fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t52943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8cfce0000","size":5,"soft_fail":false,"target_va":5411405744,"va":5411352796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428b8180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t52939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8cbce0000","size":5,"soft_fail":false,"target_va":5411406208,"va":5411353264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428b8350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t53237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8f5cf0000","size":5,"soft_fail":false,"target_va":5411406672,"va":5411353430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t5266888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8c85d5000","size":5,"soft_fail":false,"target_va":5416620769,"va":5411353876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t41426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e8d2a10000","size":5,"soft_fail":false,"target_va":5411394480,"va":5411353049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::color_id::StoneFoundationColorIds::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30448820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411352720,"raw_hex":"e84c632ffe","size":5,"soft_fail":false,"target_va":5380904880,"va":5411353695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4017,"record_offset":3484,"parent_record_offset":3020,"ancestor_record_offset":3020,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":4017,"record_offset":3164,"parent_record_offset":3020,"ancestor_record_offset":3020,"inlinee":35557,"source_line":169,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4017,"record_offset":3356,"parent_record_offset":3332,"ancestor_record_offset":3020,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4017,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":3020,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4017,"record_offset":3668,"parent_record_offset":3644,"ancestor_record_offset":3020,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4017,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":3020,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4017,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":3020,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4017,"record_offset":3716,"parent_record_offset":3692,"ancestor_record_offset":3020,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4017,"record_offset":3332,"parent_record_offset":3300,"ancestor_record_offset":3020,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4017,"record_offset":3644,"parent_record_offset":3556,"ancestor_record_offset":3020,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4017,"record_offset":3300,"parent_record_offset":3260,"ancestor_record_offset":3020,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4017,"record_offset":3260,"parent_record_offset":3164,"ancestor_record_offset":3020,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4017,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":3020,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4017,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":3020,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4017,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":3020,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":4017,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":3020,"inlinee":9288,"source_line":372,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4017,"record_offset":3612,"parent_record_offset":3584,"ancestor_record_offset":3020,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4017,"record_offset":3448,"parent_record_offset":3260,"ancestor_record_offset":3020,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=git_dependency_definition; count=7
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/bytes.rs
//     evidence={"module":4017,"record_offset":3204,"parent_record_offset":3164,"ancestor_record_offset":3020,"inlinee":9287,"source_line":8,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::set_wall_color_id::set_wall_color_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4017,"record_offset":268,"rva":42642176,"va":5411351296}
// 编译实例 RVAs: 0x28aab00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b30
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33231542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411351296,"raw_hex":"e84aed04fe","size":5,"soft_fail":false,"target_va":5378120496,"va":5411352033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb520
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33225320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411351296,"raw_hex":"e8980505fe","size":5,"soft_fail":false,"target_va":5378127136,"va":5411352451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28027223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411351296,"raw_hex":"e8a95654fe","size":5,"soft_fail":false,"target_va":5383325056,"va":5411352274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c1d50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25071703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411351296,"raw_hex":"e8a96f81fe","size":5,"soft_fail":false,"target_va":5386280272,"va":5411351970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4998030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411351296,"raw_hex":"e88e434c00","size":5,"soft_fail":false,"target_va":5416350384,"va":5411352349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4998538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411351296,"raw_hex":"e88a454c00","size":5,"soft_fail":false,"target_va":5416351056,"va":5411352513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::color_id::WallColorTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30447364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411351296,"raw_hex":"e8fc682ffe","size":5,"soft_fail":false,"target_va":5380904528,"va":5411351887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::color_id::WallColorTable::set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30447176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411351296,"raw_hex":"e8b8692ffe","size":5,"soft_fail":false,"target_va":5380904752,"va":5411351923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30117645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411351296,"raw_hex":"e8f37034fe","size":5,"soft_fail":false,"target_va":5381234608,"va":5411352248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30448462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411351296,"raw_hex":"e8b2642ffe","size":5,"soft_fail":false,"target_va":5380903536,"va":5411351993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_render_slots::WallRenderSlots::slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31706760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411351296,"raw_hex":"e878311cfe","size":5,"soft_fail":false,"target_va":5379645184,"va":5411351939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-28049993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411351296,"raw_hex":"e8b7fd53fe","size":5,"soft_fail":false,"target_va":5383302240,"va":5411352228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4017,"record_offset":1476,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4017,"record_offset":1192,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":4017,"record_offset":428,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":27034,"source_line":102,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":4017,"record_offset":644,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":4017,"record_offset":1880,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4017,"record_offset":2236,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":4017,"record_offset":1532,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":36143,"source_line":1082,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4017,"record_offset":2100,"parent_record_offset":2068,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4017,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4017,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4017,"record_offset":2068,"parent_record_offset":2028,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4017,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":268,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4017,"record_offset":2000,"parent_record_offset":1964,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4017,"record_offset":2028,"parent_record_offset":2000,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4017,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4017,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4017,"record_offset":716,"parent_record_offset":692,"ancestor_record_offset":268,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4017,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":268,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4017,"record_offset":492,"parent_record_offset":452,"ancestor_record_offset":268,"inlinee":27037,"source_line":120,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4017,"record_offset":668,"parent_record_offset":644,"ancestor_record_offset":268,"inlinee":9341,"source_line":79,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4017,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":268,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4017,"record_offset":548,"parent_record_offset":492,"ancestor_record_offset":268,"inlinee":27038,"source_line":265,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4017,"record_offset":1964,"parent_record_offset":1904,"ancestor_record_offset":268,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":4017,"record_offset":452,"parent_record_offset":428,"ancestor_record_offset":268,"inlinee":27036,"source_line":1057,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4017,"record_offset":604,"parent_record_offset":492,"ancestor_record_offset":268,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4017,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":268,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4017,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":268,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4017,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":268,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":4017,"record_offset":1648,"parent_record_offset":1612,"ancestor_record_offset":268,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":4017,"record_offset":1612,"parent_record_offset":1556,"ancestor_record_offset":268,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4017,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":268,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4017,"record_offset":520,"parent_record_offset":492,"ancestor_record_offset":268,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4017,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":268,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4017,"record_offset":1324,"parent_record_offset":1244,"ancestor_record_offset":268,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4017,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4017,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4017,"record_offset":1728,"parent_record_offset":1684,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4017,"record_offset":2188,"parent_record_offset":2028,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$40::slice_contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4017,"record_offset":2264,"parent_record_offset":2236,"ancestor_record_offset":268,"inlinee":26259,"source_line":405,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4017,"record_offset":812,"parent_record_offset":716,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4017,"record_offset":1036,"parent_record_offset":716,"ancestor_record_offset":268,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4017,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":268,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4017,"record_offset":836,"parent_record_offset":812,"ancestor_record_offset":268,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4017,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4017,"record_offset":2292,"parent_record_offset":2264,"ancestor_record_offset":268,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4017,"record_offset":1400,"parent_record_offset":1348,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4017,"record_offset":1836,"parent_record_offset":1556,"ancestor_record_offset":268,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4017,"record_offset":1800,"parent_record_offset":1556,"ancestor_record_offset":268,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/04017.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":4017,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":268,"inlinee":35826,"source_line":92,"archive":"_pdb/C/modules/04017.jsonl.gz"}
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
// 完整结构及原始类型记录: set_wall_color_id.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: set_wall_color_id.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: set_wall_color_id.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::CreatePlasterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::DeletePlasterCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::CreatePlasterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::DeletePlasterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_color::set_wall_color_id::init_color_ids_buffer
// 信息 ID: 5f77e701944f0c10b009589b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 308e88593d87089a280329f2: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)> > @ 0x2160423 -> 0x28ab090 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)> @ 0x2186276 -> 0x28ab090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::set_wall_color_id::set_wall_color_id
// 信息 ID: 45e8d0a9e07645eeaa2af2d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e4ea479c8bfbcc1ce0c023e2: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>)> > @ 0x2164a98 -> 0x28aab00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_render_slots::WallRenderSlots>)> @ 0x2186843 -> 0x28aab00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
