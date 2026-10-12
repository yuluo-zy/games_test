// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/asset/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 10d215e74447f5d5f25aeadb3f3170bda64a74306cf676ade8387e8bfbe57b38
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3085, 3167, 4090, 4589, 4590, 4605, 4746, 4747
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.9.0"}]
// use ron; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.9.0"},{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree_walker
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3167,"record_offset":3468,"rva":34442496,"va":5403151616}
// 编译实例 RVAs: 0x20d8d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$relative_path..RelativePathBuf$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17884264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8981beffe","size":5,"soft_fail":false,"target_va":5385271792,"va":5403156051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20444451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8dd0ac8fe","size":5,"soft_fail":false,"target_va":5382708864,"va":5403153310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20389164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8d4e2c8fe","size":5,"soft_fail":false,"target_va":5382768048,"va":5403157207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28475848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8387e4dfe","size":5,"soft_fail":false,"target_va":5374679664,"va":5403155507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e69f0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28265227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8f5b450fe","size":5,"soft_fail":false,"target_va":5374896624,"va":5403161846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=38
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28070374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e81aae53fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5403151873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140889e60
//     owner=unresolved_namespace; count=7
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25493737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e817ff7afe","size":5,"soft_fail":false,"target_va":5377662560,"va":5403156292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b770c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22424480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e860d4a9fe","size":5,"soft_fail":false,"target_va":5380731072,"va":5403155547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20448739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e81dfac7fe","size":5,"soft_fail":false,"target_va":5382704752,"va":5403153486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=8
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20324020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e84ce1c9fe","size":5,"soft_fail":false,"target_va":5382827728,"va":5403151743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d8d6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20235066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8c63ccbfe","size":5,"soft_fail":false,"target_va":5382919872,"va":5403154933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d4ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e854b5ffff","size":5,"soft_fail":false,"target_va":5403135136,"va":5403154247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142421c80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3440304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8b07e3400","size":5,"soft_fail":false,"target_va":5406596224,"va":5403155915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142421dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3441561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e899833400","size":5,"soft_fail":false,"target_va":5406596560,"va":5403154994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142423660
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3446581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e835973400","size":5,"soft_fail":false,"target_va":5406602848,"va":5403156262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142425b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3459030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8d6c73400","size":5,"soft_fail":false,"target_va":5406612224,"va":5403153189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142425dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3460711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e867ce3400","size":5,"soft_fail":false,"target_va":5406612944,"va":5403152228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8693997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8eda88400","size":5,"soft_fail":false,"target_va":5411847168,"va":5403153166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-20312654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8b20dcafe","size":5,"soft_fail":false,"target_va":5382840208,"va":5403152857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-28796499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8ad9948fe","size":5,"soft_fail":false,"target_va":5374358848,"va":5403155342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-20470832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8d0a3c7fe","size":5,"soft_fail":false,"target_va":5382683584,"va":5403154411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8628555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e84ba98300","size":5,"soft_fail":false,"target_va":5411786032,"va":5403157472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h99f34d5a19ceb942 (.llvm.4711583917080959974)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8663710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e89e328400","size":5,"soft_fail":false,"target_va":5411816640,"va":5403152925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8657901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8ed1b8400","size":5,"soft_fail":false,"target_va":5411815824,"va":5403157918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8665330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8f2388400","size":5,"soft_fail":false,"target_va":5411817328,"va":5403151993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20504100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8dc21c7fe","size":5,"soft_fail":false,"target_va":5382655024,"va":5403159119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::texture::config::InheritableTextureOpts::into_concrete
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3654061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8adc13700","size":5,"soft_fail":false,"target_va":5406810704,"va":5403156638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::texture::scan::scan_asset_tree_walker
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e84cf6ffff","size":5,"soft_fail":false,"target_va":5403151616,"va":5403154095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8383262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e81eeb7f00","size":5,"soft_fail":false,"target_va":5411541776,"va":5403158509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8383656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8a8ec7f00","size":5,"soft_fail":false,"target_va":5411542296,"va":5403158635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$asset..texture..desc..TextureAssetDesc$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8f59dffff","size":5,"soft_fail":false,"target_va":5403136096,"va":5403161190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8384300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e82cef7f00","size":5,"soft_fail":false,"target_va":5411542880,"va":5403158575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTable$LT$T$C$A$GT$::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3453167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8efb03400","size":5,"soft_fail":false,"target_va":5406608192,"va":5403155020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e80b4d0000","size":5,"soft_fail":false,"target_va":5403174032,"va":5403154304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::RelativePath::join
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3458408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e868c53400","size":5,"soft_fail":false,"target_va":5406612592,"va":5403154179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20443913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8f70cc8fe","size":5,"soft_fail":false,"target_va":5382709552,"va":5403153460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20443968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8c00cc8fe","size":5,"soft_fail":false,"target_va":5382709424,"va":5403153387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-20444725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8cb09c8fe","size":5,"soft_fail":false,"target_va":5382709008,"va":5403153728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20446492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8e402c8fe","size":5,"soft_fail":false,"target_va":5382705472,"va":5403151959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20494050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e81e49c7fe","size":5,"soft_fail":false,"target_va":5382657856,"va":5403151901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20407660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8949ac8fe","size":5,"soft_fail":false,"target_va":5382744080,"va":5403151735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20409795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e83d92c8fe","size":5,"soft_fail":false,"target_va":5382743968,"va":5403153758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_stem
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20411574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e84a8bc8fe","size":5,"soft_fail":false,"target_va":5382743808,"va":5403155377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20382550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8aafcc8fe","size":5,"soft_fail":false,"target_va":5382770560,"va":5403153105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20375174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e87a19c9fe","size":5,"soft_fail":false,"target_va":5382776592,"va":5403151761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20398219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e875bfc8fe","size":5,"soft_fail":false,"target_va":5382753792,"va":5403152006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5403151616,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5403151844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3167,"record_offset":20396,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3167,"record_offset":13568,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":10360,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":13280,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":15320,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":3167,"record_offset":7076,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":12467,"source_line":58,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::config::InheritableTextureOpts::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":3167,"record_offset":6700,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":108136,"source_line":33,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::config::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":3167,"record_offset":4836,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":108133,"source_line":7,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::scan::load_texture_group_config
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/scan.rs
//     evidence={"module":3167,"record_offset":5732,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":108135,"source_line":162,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3167,"record_offset":19244,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3167,"record_offset":13484,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":9324,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":14364,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":5145,"source_line":1277,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":7720,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":15288,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":8900,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":7388,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":18936,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3167,"record_offset":7756,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":9143,"source_line":2397,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3167,"record_offset":9456,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":15416,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":16436,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":10388,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4955,"source_line":605,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":13984,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":7952,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":7688,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsString::as_os_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":7604,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":46522,"source_line":202,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":8700,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":108143,"source_line":698,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3167,"record_offset":7492,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":16199,"source_line":1622,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3167,"record_offset":7324,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3167,"record_offset":3892,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":13003,"source_line":3025,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3167,"record_offset":3724,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3167,"record_offset":3752,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":4452,"parent_record_offset":4428,"ancestor_record_offset":3468,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":4428,"parent_record_offset":4340,"ancestor_record_offset":3468,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3167,"record_offset":4340,"parent_record_offset":4312,"ancestor_record_offset":3468,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3167,"record_offset":7140,"parent_record_offset":7116,"ancestor_record_offset":3468,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3167,"record_offset":7188,"parent_record_offset":7164,"ancestor_record_offset":3468,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":13364,"parent_record_offset":13332,"ancestor_record_offset":3468,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":6108,"parent_record_offset":6080,"ancestor_record_offset":3468,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":3468,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":13412,"parent_record_offset":13388,"ancestor_record_offset":3468,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":13388,"parent_record_offset":13364,"ancestor_record_offset":3468,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":3468,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3167,"record_offset":13656,"parent_record_offset":13604,"ancestor_record_offset":3468,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3167,"record_offset":13604,"parent_record_offset":13568,"ancestor_record_offset":3468,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3167,"record_offset":11624,"parent_record_offset":11588,"ancestor_record_offset":3468,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3167,"record_offset":11588,"parent_record_offset":11564,"ancestor_record_offset":3468,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":13332,"parent_record_offset":13280,"ancestor_record_offset":3468,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":3468,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":3167,"record_offset":7252,"parent_record_offset":7228,"ancestor_record_offset":3468,"inlinee":12447,"source_line":316,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":3167,"record_offset":7228,"parent_record_offset":7076,"ancestor_record_offset":3468,"inlinee":12446,"source_line":33,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3167,"record_offset":6764,"parent_record_offset":6700,"ancestor_record_offset":3468,"inlinee":108137,"source_line":30,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3167,"record_offset":6800,"parent_record_offset":6764,"ancestor_record_offset":3468,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::scan::scan_asset_tree_walker::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/scan.rs
//     evidence={"module":3167,"record_offset":7116,"parent_record_offset":7076,"ancestor_record_offset":3468,"inlinee":108141,"source_line":34,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::scan::scan_asset_tree_walker::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/scan.rs
//     evidence={"module":3167,"record_offset":14400,"parent_record_offset":14364,"ancestor_record_offset":3468,"inlinee":108145,"source_line":72,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::scan::scan_asset_tree_walker::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/scan.rs
//     evidence={"module":3167,"record_offset":9356,"parent_record_offset":9324,"ancestor_record_offset":3468,"inlinee":108144,"source_line":110,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3167,"record_offset":4372,"parent_record_offset":4340,"ancestor_record_offset":3468,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3167,"record_offset":5416,"parent_record_offset":5384,"ancestor_record_offset":3468,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3167,"record_offset":5476,"parent_record_offset":5444,"ancestor_record_offset":3468,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3167,"record_offset":13532,"parent_record_offset":13508,"ancestor_record_offset":3468,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3167,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":3468,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3167,"record_offset":15724,"parent_record_offset":15700,"ancestor_record_offset":3468,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3167,"record_offset":15808,"parent_record_offset":15784,"ancestor_record_offset":3468,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3167,"record_offset":14424,"parent_record_offset":14400,"ancestor_record_offset":3468,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3167,"record_offset":13684,"parent_record_offset":13604,"ancestor_record_offset":3468,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3167,"record_offset":15748,"parent_record_offset":15724,"ancestor_record_offset":3468,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3167,"record_offset":16260,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3167,"record_offset":13508,"parent_record_offset":13484,"ancestor_record_offset":3468,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3167,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":3468,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3167,"record_offset":4396,"parent_record_offset":4372,"ancestor_record_offset":3468,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3167,"record_offset":20084,"parent_record_offset":20056,"ancestor_record_offset":3468,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3167,"record_offset":5444,"parent_record_offset":5384,"ancestor_record_offset":3468,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3167,"record_offset":15960,"parent_record_offset":15932,"ancestor_record_offset":3468,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3167,"record_offset":15932,"parent_record_offset":15904,"ancestor_record_offset":3468,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3167,"record_offset":5020,"parent_record_offset":4980,"ancestor_record_offset":3468,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":3468,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":9412,"parent_record_offset":9356,"ancestor_record_offset":3468,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":5136,"parent_record_offset":5108,"ancestor_record_offset":3468,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":6828,"parent_record_offset":6800,"ancestor_record_offset":3468,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=180
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":3996,"parent_record_offset":3892,"ancestor_record_offset":3468,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":5224,"parent_record_offset":5108,"ancestor_record_offset":3468,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":3468,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":4140,"parent_record_offset":4112,"ancestor_record_offset":3468,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":3468,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3468,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3167,"record_offset":11656,"parent_record_offset":11624,"ancestor_record_offset":3468,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":5300,"parent_record_offset":5272,"ancestor_record_offset":3468,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":3468,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":5328,"parent_record_offset":5272,"ancestor_record_offset":3468,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":13712,"parent_record_offset":13604,"ancestor_record_offset":3468,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":3960,"parent_record_offset":3932,"ancestor_record_offset":3468,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":5824,"parent_record_offset":5732,"ancestor_record_offset":3468,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":5860,"parent_record_offset":5732,"ancestor_record_offset":3468,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3167,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":3468,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3167,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":3468,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3167,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":3468,"inlinee":5361,"source_line":2581,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3167,"record_offset":15348,"parent_record_offset":15320,"ancestor_record_offset":3468,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3167,"record_offset":15376,"parent_record_offset":15348,"ancestor_record_offset":3468,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3167,"record_offset":7804,"parent_record_offset":7780,"ancestor_record_offset":3468,"inlinee":8748,"source_line":981,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3167,"record_offset":7780,"parent_record_offset":7756,"ancestor_record_offset":3468,"inlinee":9146,"source_line":1013,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3167,"record_offset":11684,"parent_record_offset":11624,"ancestor_record_offset":3468,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3167,"record_offset":11716,"parent_record_offset":11588,"ancestor_record_offset":3468,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3167,"record_offset":15868,"parent_record_offset":15840,"ancestor_record_offset":3468,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3167,"record_offset":15904,"parent_record_offset":15868,"ancestor_record_offset":3468,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3167,"record_offset":15840,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3167,"record_offset":15700,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3167,"record_offset":16288,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3167,"record_offset":15784,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":3167,"record_offset":15672,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":3167,"record_offset":16196,"parent_record_offset":16172,"ancestor_record_offset":3468,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":15448,"parent_record_offset":15416,"ancestor_record_offset":3468,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":15480,"parent_record_offset":15448,"ancestor_record_offset":3468,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":3468,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":14008,"parent_record_offset":13984,"ancestor_record_offset":3468,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":14040,"parent_record_offset":14008,"ancestor_record_offset":3468,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":16172,"parent_record_offset":16004,"ancestor_record_offset":3468,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":14068,"parent_record_offset":14040,"ancestor_record_offset":3468,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":16112,"parent_record_offset":16028,"ancestor_record_offset":3468,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":16056,"parent_record_offset":16028,"ancestor_record_offset":3468,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":16380,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":16028,"parent_record_offset":16004,"ancestor_record_offset":3468,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":15564,"parent_record_offset":15540,"ancestor_record_offset":3468,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":15540,"parent_record_offset":15480,"ancestor_record_offset":3468,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":15588,"parent_record_offset":15564,"ancestor_record_offset":3468,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":15644,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":10828,"parent_record_offset":10804,"ancestor_record_offset":3468,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":16004,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":3167,"record_offset":5888,"parent_record_offset":5732,"ancestor_record_offset":3468,"inlinee":13006,"source_line":108,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":3167,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":3468,"inlinee":12914,"source_line":51,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":8668,"parent_record_offset":7952,"ancestor_record_offset":3468,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":4868,"parent_record_offset":4836,"ancestor_record_offset":3468,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":9384,"parent_record_offset":9356,"ancestor_record_offset":3468,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$44::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":8804,"parent_record_offset":8780,"ancestor_record_offset":3468,"inlinee":12384,"source_line":1427,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$45::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":8780,"parent_record_offset":8700,"ancestor_record_offset":3468,"inlinee":46232,"source_line":1435,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":3800,"parent_record_offset":3776,"ancestor_record_offset":3468,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":3468,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":3824,"parent_record_offset":3800,"ancestor_record_offset":3468,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3167,"record_offset":7516,"parent_record_offset":7492,"ancestor_record_offset":3468,"inlinee":13008,"source_line":2036,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3167,"record_offset":3932,"parent_record_offset":3892,"ancestor_record_offset":3468,"inlinee":12458,"source_line":2381,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3167,"record_offset":5792,"parent_record_offset":5732,"ancestor_record_offset":3468,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3167,"record_offset":4924,"parent_record_offset":4896,"ancestor_record_offset":3468,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3167,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":3468,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3167,"record_offset":5048,"parent_record_offset":5020,"ancestor_record_offset":3468,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3167,"record_offset":5384,"parent_record_offset":4980,"ancestor_record_offset":3468,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3167,"record_offset":4896,"parent_record_offset":4868,"ancestor_record_offset":3468,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":3167,"record_offset":4112,"parent_record_offset":4080,"ancestor_record_offset":3468,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":3167,"record_offset":4080,"parent_record_offset":4052,"ancestor_record_offset":3468,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3167,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":3468,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":3167,"record_offset":7540,"parent_record_offset":7516,"ancestor_record_offset":3468,"inlinee":13010,"source_line":1126,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":3167,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":3468,"inlinee":13011,"source_line":1132,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3167,"record_offset":5076,"parent_record_offset":5048,"ancestor_record_offset":3468,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3167,"record_offset":5108,"parent_record_offset":5076,"ancestor_record_offset":3468,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3167,"record_offset":4980,"parent_record_offset":4952,"ancestor_record_offset":3468,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3167,"record_offset":4952,"parent_record_offset":4924,"ancestor_record_offset":3468,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::Deserializer::handle_any_struct<serde::__private::de::content::ContentVisitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4589,"record_offset":732,"rva":40360320,"va":5409069440}
// 编译实例 RVAs: 0x267d980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33988095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409069440,"raw_hex":"e80162f9fd","size":5,"soft_fail":false,"target_va":5375081488,"va":5409069578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14267f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409069440,"raw_hex":"e8681a0000","size":5,"soft_fail":false,"target_va":5409076592,"va":5409069827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142695450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t96740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409069440,"raw_hex":"e8e4790100","size":5,"soft_fail":false,"target_va":5409166416,"va":5409069671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2777421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409069440,"raw_hex":"e84d612a00","size":5,"soft_fail":false,"target_va":5411847168,"va":5409069742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2470928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409069440,"raw_hex":"e810b42500","size":5,"soft_fail":false,"target_va":5411540963,"va":5409070030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<serde::__private::de::content::Content>,enum2$<ron::error::Error> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409069440,"raw_hex":"e8e7020000","size":5,"soft_fail":false,"target_va":5409070752,"va":5409070004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_struct_type
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28004924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409069440,"raw_hex":"e8c4ad54fe","size":5,"soft_fail":false,"target_va":5381064592,"va":5409069511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28023098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409069440,"raw_hex":"e8c66654fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5409069797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-27998646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409069440,"raw_hex":"e84ac654fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5409069809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] ron::de::Deserializer::handle_struct_after_name<serde::__private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tjmp\t384","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5409069440,"raw_hex":"e980010000","size":5,"soft_fail":false,"target_va":5409070112,"va":5409069723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] ron::de::impl$2::deserialize_tuple<serde::__private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tjmp\t18099","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5409069440,"raw_hex":"e9b3460000","size":5,"soft_fail":false,"target_va":5409088016,"va":5409069912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":916,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":1696,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$4::visit_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4589,"record_offset":948,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":6510,"source_line":419,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$4::visit_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4589,"record_offset":1724,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":19161,"source_line":461,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":732,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":732,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":1504,"parent_record_offset":1328,"ancestor_record_offset":732,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":1268,"parent_record_offset":1240,"ancestor_record_offset":732,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":1328,"parent_record_offset":1296,"ancestor_record_offset":732,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":1296,"parent_record_offset":1268,"ancestor_record_offset":732,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":1364,"parent_record_offset":1328,"ancestor_record_offset":732,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":1204,"parent_record_offset":1172,"ancestor_record_offset":732,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":1172,"parent_record_offset":1140,"ancestor_record_offset":732,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":1108,"parent_record_offset":1076,"ancestor_record_offset":732,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":1140,"parent_record_offset":1108,"ancestor_record_offset":732,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":1076,"parent_record_offset":1044,"ancestor_record_offset":732,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4589,"record_offset":1044,"parent_record_offset":1012,"ancestor_record_offset":732,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4589,"record_offset":1012,"parent_record_offset":980,"ancestor_record_offset":732,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4589,"record_offset":1240,"parent_record_offset":1204,"ancestor_record_offset":732,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4589,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":732,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4589,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":732,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":4589,"record_offset":980,"parent_record_offset":948,"ancestor_record_offset":732,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4589,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":732,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4589,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":732,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":732,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4589,"record_offset":1604,"parent_record_offset":1204,"ancestor_record_offset":732,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_any<serde::__private::de::content::ContentVisitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4589,"record_offset":12972,"rva":40367472,"va":5409076592}
// 编译实例 RVAs: 0x267f570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142695360
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t89053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e8dd5b0100","size":5,"soft_fail":false,"target_va":5409166176,"va":5409077118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e825e4ffff","size":5,"soft_fail":false,"target_va":5409070784,"va":5409077910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e803e5ffff","size":5,"soft_fail":false,"target_va":5409070992,"va":5409077896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::Deserializer::handle_any_struct<serde::__private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-8444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e804dfffff","size":5,"soft_fail":false,"target_va":5409069440,"va":5409077879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::Deserializer::handle_struct_after_name<serde::__private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-6653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e803e6ffff","size":5,"soft_fail":false,"target_va":5409070112,"va":5409076760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_byte_buf<serde::__private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t38958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e82e980000","size":5,"soft_fail":false,"target_va":5409117216,"va":5409078253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_char<serde::__private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-8636322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e85e387cff","size":5,"soft_fail":false,"target_va":5400441952,"va":5409078269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_map<serde::__private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t6679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e8171a0000","size":5,"soft_fail":false,"target_va":5409084896,"va":5409078212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_seq<serde::__private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t7534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e86e1d0000","size":5,"soft_fail":false,"target_va":5409085712,"va":5409078173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_str<serde::__private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-8636902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e81a367cff","size":5,"soft_fail":false,"target_va":5400441184,"va":5409078081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_tuple<serde::__private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t11185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e8b12b0000","size":5,"soft_fail":false,"target_va":5409088016,"va":5409076826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::any_number
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28019946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e8167354fe","size":5,"soft_fail":false,"target_va":5381058032,"va":5409077973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28030544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e8b04954fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5409076651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_ident
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28012735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e8418f54fe","size":5,"soft_fail":false,"target_va":5381064208,"va":5409076938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_struct_type
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28012136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e8989154fe","size":5,"soft_fail":false,"target_va":5381064592,"va":5409076723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28013194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e8768d54fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5409077205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28030340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e87c4a54fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5409077039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_ident
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28011137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e87f9554fe","size":5,"soft_fail":false,"target_va":5381065728,"va":5409076860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28030111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e8614b54fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5409076682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28008135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e839a154fe","size":5,"soft_fail":false,"target_va":5381069696,"va":5409077826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::peek_char_or_eof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28031959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e8294454fe","size":5,"soft_fail":false,"target_va":5381045968,"va":5409077922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28005853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e823aa54fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5409077016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28031734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e80a4554fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5409078225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::value::number::Number::visit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t2089558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409076592,"raw_hex":"e856e21f00","size":5,"soft_fail":false,"target_va":5411167568,"va":5409078005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":14056,"parent_record_offset":12972,"ancestor_record_offset":12972,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":13236,"parent_record_offset":12972,"ancestor_record_offset":12972,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":13272,"parent_record_offset":12972,"ancestor_record_offset":12972,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4589,"record_offset":14280,"parent_record_offset":12972,"ancestor_record_offset":12972,"inlinee":8789,"source_line":1366,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] ron::de::impl$2::deserialize_option
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":13356,"parent_record_offset":12972,"ancestor_record_offset":12972,"inlinee":13364,"source_line":484,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] ron::de::impl$2::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":14252,"parent_record_offset":12972,"ancestor_record_offset":12972,"inlinee":5162,"source_line":455,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] ron::de::impl$2::deserialize_unit
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":13176,"parent_record_offset":12972,"ancestor_record_offset":12972,"inlinee":13363,"source_line":520,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$4::visit_bool
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4589,"record_offset":13300,"parent_record_offset":12972,"ancestor_record_offset":12972,"inlinee":19164,"source_line":335,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$4::visit_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4589,"record_offset":13916,"parent_record_offset":12972,"ancestor_record_offset":12972,"inlinee":100072,"source_line":398,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$4::visit_f64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4589,"record_offset":13944,"parent_record_offset":12972,"ancestor_record_offset":12972,"inlinee":19166,"source_line":405,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$4::visit_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4589,"record_offset":13888,"parent_record_offset":12972,"ancestor_record_offset":12972,"inlinee":19161,"source_line":461,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4589,"record_offset":14352,"parent_record_offset":14328,"ancestor_record_offset":12972,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":13664,"parent_record_offset":13356,"ancestor_record_offset":12972,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":13732,"parent_record_offset":13356,"ancestor_record_offset":12972,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4589,"record_offset":14088,"parent_record_offset":14056,"ancestor_record_offset":12972,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":13512,"parent_record_offset":13356,"ancestor_record_offset":12972,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":13856,"parent_record_offset":13356,"ancestor_record_offset":12972,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":14376,"parent_record_offset":14352,"ancestor_record_offset":12972,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":14400,"parent_record_offset":14376,"ancestor_record_offset":12972,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4589,"record_offset":14328,"parent_record_offset":14304,"ancestor_record_offset":12972,"inlinee":5361,"source_line":2581,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4589,"record_offset":14304,"parent_record_offset":14280,"ancestor_record_offset":12972,"inlinee":8748,"source_line":981,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::Extensions::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4589,"record_offset":13568,"parent_record_offset":13356,"ancestor_record_offset":12972,"inlinee":12958,"source_line":703,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4589,"record_offset":13624,"parent_record_offset":13596,"ancestor_record_offset":12972,"inlinee":13353,"source_line":641,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4589,"record_offset":13596,"parent_record_offset":13568,"ancestor_record_offset":12972,"inlinee":12959,"source_line":703,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$4::visit_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4589,"record_offset":13204,"parent_record_offset":13176,"ancestor_record_offset":12972,"inlinee":19161,"source_line":461,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_byte_buf<serde::__private::de::content::ContentVisitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4589,"record_offset":57528,"rva":40408096,"va":5409117216}
// 编译实例 RVAs: 0x2689420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde..de..impls..$LT$impl$u20$serde..de..Deserialize$u20$for$u20$alloc..vec..Vec$LT$T$GT$$GT$..deserialize..VecVisitor$LT$T$GT$$u20$as$u20$serde..de..Visitor$GT$::visit_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2034553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409117216,"raw_hex":"e8790b1f00","size":5,"soft_fail":false,"target_va":5411151904,"va":5409117346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34036255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409117216,"raw_hex":"e8e1a5f8fd","size":5,"soft_fail":false,"target_va":5375081504,"va":5409117754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::byte_string
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28044862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409117216,"raw_hex":"e8c21154fe","size":5,"soft_fail":false,"target_va":5381072640,"va":5409117497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28071158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409117216,"raw_hex":"e80aab53fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5409117265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28070591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409117216,"raw_hex":"e841ad53fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5409117290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28046257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409117216,"raw_hex":"e84f0c54fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5409117420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":58596,"parent_record_offset":57528,"ancestor_record_offset":57528,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":58660,"parent_record_offset":57528,"ancestor_record_offset":57528,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$4::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4589,"record_offset":58632,"parent_record_offset":57528,"ancestor_record_offset":57528,"inlinee":100084,"source_line":447,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$4::visit_byte_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4589,"record_offset":58568,"parent_record_offset":57528,"ancestor_record_offset":57528,"inlinee":100083,"source_line":454,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":57712,"parent_record_offset":57528,"ancestor_record_offset":57528,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":58112,"parent_record_offset":58088,"ancestor_record_offset":57528,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":58088,"parent_record_offset":58032,"ancestor_record_offset":57528,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":58060,"parent_record_offset":58032,"ancestor_record_offset":57528,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":58032,"parent_record_offset":58008,"ancestor_record_offset":57528,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":58008,"parent_record_offset":57984,"ancestor_record_offset":57528,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":57820,"parent_record_offset":57744,"ancestor_record_offset":57528,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":57896,"parent_record_offset":57744,"ancestor_record_offset":57528,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4589,"record_offset":57960,"parent_record_offset":57744,"ancestor_record_offset":57528,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":57924,"parent_record_offset":57744,"ancestor_record_offset":57528,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::CommaSeparated::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":57852,"parent_record_offset":57744,"ancestor_record_offset":57528,"inlinee":13279,"source_line":791,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_seq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":57744,"parent_record_offset":57712,"ancestor_record_offset":57528,"inlinee":13417,"source_line":595,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4589,"record_offset":38780,"rva":40388288,"va":5409097408}
// 编译实例 RVAs: 0x26846c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_field
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2382015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409097408,"raw_hex":"e8bf582400","size":5,"soft_fail":false,"target_va":5411481328,"va":5409099308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b742a0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28380016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409097408,"raw_hex":"e890f44efe","size":5,"soft_fail":false,"target_va":5380719264,"va":5409099275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14267fc40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409097408,"raw_hex":"e828afffff","size":5,"soft_fail":false,"target_va":5409078336,"va":5409099027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287d4f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2067445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409097408,"raw_hex":"e8f58b1f00","size":5,"soft_fail":false,"target_va":5411165424,"va":5409097974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28012332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409097408,"raw_hex":"e8d49054fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5409097847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::struct_error_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28014837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409097408,"raw_hex":"e80b8754fe","size":5,"soft_fail":false,"target_va":5381085680,"va":5409100512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28035134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409097408,"raw_hex":"e8c23754fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5409099145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28050958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409097408,"raw_hex":"e8f2f953fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5409097657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_struct_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28031701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409097408,"raw_hex":"e82b4554fe","size":5,"soft_fail":false,"target_va":5381065872,"va":5409097568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28028792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409097408,"raw_hex":"e8885054fe","size":5,"soft_fail":false,"target_va":5381069696,"va":5409098483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28026449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409097408,"raw_hex":"e8af5954fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5409097612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2068556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409097408,"raw_hex":"e84c901f00","size":5,"soft_fail":false,"target_va":5411168768,"va":5409100207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":38968,"parent_record_offset":38780,"ancestor_record_offset":38780,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":42028,"parent_record_offset":38780,"ancestor_record_offset":38780,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] ron::de::Deserializer::handle_struct_after_name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":39040,"parent_record_offset":38780,"ancestor_record_offset":38780,"inlinee":13303,"source_line":225,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":40060,"parent_record_offset":40032,"ancestor_record_offset":38780,"inlinee":134082,"source_line":71,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":39640,"parent_record_offset":39616,"ancestor_record_offset":38780,"inlinee":134080,"source_line":27,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":39260,"parent_record_offset":39040,"ancestor_record_offset":38780,"inlinee":134078,"source_line":27,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4589,"record_offset":40112,"parent_record_offset":40088,"ancestor_record_offset":38780,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":39200,"parent_record_offset":39040,"ancestor_record_offset":38780,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":39672,"parent_record_offset":39356,"ancestor_record_offset":38780,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4589,"record_offset":40364,"parent_record_offset":39260,"ancestor_record_offset":38780,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4589,"record_offset":40840,"parent_record_offset":40436,"ancestor_record_offset":38780,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":40252,"parent_record_offset":39356,"ancestor_record_offset":38780,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":41792,"parent_record_offset":39040,"ancestor_record_offset":38780,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":39508,"parent_record_offset":39356,"ancestor_record_offset":38780,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":40212,"parent_record_offset":39952,"ancestor_record_offset":38780,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":40136,"parent_record_offset":40112,"ancestor_record_offset":38780,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":40160,"parent_record_offset":40136,"ancestor_record_offset":38780,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4589,"record_offset":40088,"parent_record_offset":40060,"ancestor_record_offset":38780,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4589,"record_offset":40912,"parent_record_offset":40888,"ancestor_record_offset":38780,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::CommaSeparated::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":39232,"parent_record_offset":39040,"ancestor_record_offset":38780,"inlinee":13279,"source_line":791,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":4589,"record_offset":39576,"parent_record_offset":39356,"ancestor_record_offset":38780,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$1::handle_struct_after_name::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":41816,"parent_record_offset":41792,"ancestor_record_offset":38780,"inlinee":13282,"source_line":240,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":39952,"parent_record_offset":39920,"ancestor_record_offset":38780,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":39356,"parent_record_offset":39320,"ancestor_record_offset":38780,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":40436,"parent_record_offset":40392,"ancestor_record_offset":38780,"inlinee":13322,"source_line":857,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":40672,"parent_record_offset":40648,"ancestor_record_offset":38780,"inlinee":100077,"source_line":214,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":39320,"parent_record_offset":39260,"ancestor_record_offset":38780,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":40392,"parent_record_offset":39260,"ancestor_record_offset":38780,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":40032,"parent_record_offset":39952,"ancestor_record_offset":38780,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":39616,"parent_record_offset":39356,"ancestor_record_offset":38780,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":40648,"parent_record_offset":40624,"ancestor_record_offset":38780,"inlinee":73646,"source_line":1505,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4589,"record_offset":41212,"parent_record_offset":39260,"ancestor_record_offset":38780,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":4589,"record_offset":41260,"parent_record_offset":41236,"ancestor_record_offset":38780,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":4589,"record_offset":41236,"parent_record_offset":41212,"ancestor_record_offset":38780,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4589,"record_offset":31216,"rva":40379360,"va":5409088480}
// 编译实例 RVAs: 0x26823e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142681ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e876eeffff","size":5,"soft_fail":false,"target_va":5409086144,"va":5409090629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142681d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e80ef0ffff","size":5,"soft_fail":false,"target_va":5409086720,"va":5409090797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142681f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e8e9f3ffff","size":5,"soft_fail":false,"target_va":5409087296,"va":5409090386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14268aee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e87f840000","size":5,"soft_fail":false,"target_va":5409124064,"va":5409090140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14268bcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e87f940000","size":5,"soft_fail":false,"target_va":5409127664,"va":5409089644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287d7a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2077045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e875b11f00","size":5,"soft_fail":false,"target_va":5411166112,"va":5409089062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28003221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e86bb454fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5409088736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::struct_error_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28005497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e887ab54fe","size":5,"soft_fail":false,"target_va":5381085680,"va":5409091172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28024876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e8d45f54fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5409088887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28041934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e8321d54fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5409088633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_struct_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28022703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e8516854fe","size":5,"soft_fail":false,"target_va":5381065872,"va":5409088570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28019913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e8377354fe","size":5,"soft_fail":false,"target_va":5381069696,"va":5409089604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28017437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409088480,"raw_hex":"e8e37c54fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5409088600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":31404,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":34436,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] ron::de::Deserializer::handle_struct_after_name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":31468,"parent_record_offset":31216,"ancestor_record_offset":31216,"inlinee":13303,"source_line":225,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4589,"record_offset":33252,"parent_record_offset":33228,"ancestor_record_offset":31216,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":32056,"parent_record_offset":32032,"ancestor_record_offset":31216,"inlinee":134065,"source_line":7,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$2::visit_map
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":31672,"parent_record_offset":31468,"ancestor_record_offset":31216,"inlinee":134063,"source_line":7,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":32876,"parent_record_offset":32852,"ancestor_record_offset":31216,"inlinee":134067,"source_line":17,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":34092,"parent_record_offset":34064,"ancestor_record_offset":31216,"inlinee":134075,"source_line":17,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":33228,"parent_record_offset":33204,"ancestor_record_offset":31216,"inlinee":134069,"source_line":17,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$3::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":33684,"parent_record_offset":33656,"ancestor_record_offset":31216,"inlinee":134073,"source_line":17,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4589,"record_offset":33276,"parent_record_offset":33252,"ancestor_record_offset":31216,"inlinee":108179,"source_line":4,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":31612,"parent_record_offset":31468,"ancestor_record_offset":31216,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":32088,"parent_record_offset":31768,"ancestor_record_offset":31216,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4589,"record_offset":32624,"parent_record_offset":31672,"ancestor_record_offset":31216,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":32508,"parent_record_offset":31768,"ancestor_record_offset":31216,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":34264,"parent_record_offset":31468,"ancestor_record_offset":31216,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":31920,"parent_record_offset":31768,"ancestor_record_offset":31216,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":32468,"parent_record_offset":32368,"ancestor_record_offset":31216,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::CommaSeparated::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":31644,"parent_record_offset":31468,"ancestor_record_offset":31216,"inlinee":13279,"source_line":791,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":4589,"record_offset":31992,"parent_record_offset":31768,"ancestor_record_offset":31216,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$1::handle_struct_after_name::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":34308,"parent_record_offset":34264,"ancestor_record_offset":31216,"inlinee":13282,"source_line":240,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":32368,"parent_record_offset":32340,"ancestor_record_offset":31216,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":31768,"parent_record_offset":31728,"ancestor_record_offset":31216,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":32676,"parent_record_offset":32652,"ancestor_record_offset":31216,"inlinee":13322,"source_line":857,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::Deserializer::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":32824,"parent_record_offset":32676,"ancestor_record_offset":31216,"inlinee":100071,"source_line":10,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":33300,"parent_record_offset":33276,"ancestor_record_offset":31216,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":31728,"parent_record_offset":31672,"ancestor_record_offset":31216,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":32652,"parent_record_offset":31672,"ancestor_record_offset":31216,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":32440,"parent_record_offset":32368,"ancestor_record_offset":31216,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":32032,"parent_record_offset":31768,"ancestor_record_offset":31216,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$5::next_element_seed<core::marker::PhantomData<u8> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4589,"record_offset":11940,"rva":40366656,"va":5409075776}
// 编译实例 RVAs: 0x267f240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14287be20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2083713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409075776,"raw_hex":"e881cb1f00","size":5,"soft_fail":false,"target_va":5411159584,"va":5409075866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-27990279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409075776,"raw_hex":"e8f9e654fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5409075794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28011920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409075776,"raw_hex":"e8709254fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5409075931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":12136,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":12300,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":12108,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":12388,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":12168,"parent_record_offset":11940,"ancestor_record_offset":11940,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":12252,"parent_record_offset":12224,"ancestor_record_offset":11940,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_u8
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":12224,"parent_record_offset":12196,"ancestor_record_offset":11940,"inlinee":13362,"source_line":388,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$63::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":12196,"parent_record_offset":12168,"ancestor_record_offset":11940,"inlinee":6547,"source_line":134,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$6::next_key_seed<core::marker::PhantomData<alloc::string::String> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4589,"record_offset":8828,"rva":40364512,"va":5409073632}
// 编译实例 RVAs: 0x267e9e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33992495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409073632,"raw_hex":"e8d150f9fd","size":5,"soft_fail":false,"target_va":5375081488,"va":5409073978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2772963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409073632,"raw_hex":"e8e34f2a00","size":5,"soft_fail":false,"target_va":5411847168,"va":5409074200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2466717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409073632,"raw_hex":"e89da32500","size":5,"soft_fail":false,"target_va":5411540963,"va":5409074241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-27988145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409073632,"raw_hex":"e84fef54fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5409073660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::string
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-27999904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409073632,"raw_hex":"e860c154fe","size":5,"soft_fail":false,"target_va":5381074000,"va":5409073899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":9044,"parent_record_offset":8828,"ancestor_record_offset":8828,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":9156,"parent_record_offset":8828,"ancestor_record_offset":8828,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":9012,"parent_record_offset":8828,"ancestor_record_offset":8828,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":9072,"parent_record_offset":8828,"ancestor_record_offset":8828,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":9216,"parent_record_offset":8828,"ancestor_record_offset":8828,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":8828,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":9976,"parent_record_offset":9952,"ancestor_record_offset":8828,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":9928,"parent_record_offset":9752,"ancestor_record_offset":8828,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":9692,"parent_record_offset":9664,"ancestor_record_offset":8828,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":9752,"parent_record_offset":9720,"ancestor_record_offset":8828,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":9720,"parent_record_offset":9692,"ancestor_record_offset":8828,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":9788,"parent_record_offset":9752,"ancestor_record_offset":8828,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":9628,"parent_record_offset":9596,"ancestor_record_offset":8828,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":9596,"parent_record_offset":9564,"ancestor_record_offset":8828,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":9532,"parent_record_offset":9500,"ancestor_record_offset":8828,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":9564,"parent_record_offset":9532,"ancestor_record_offset":8828,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":9500,"parent_record_offset":9468,"ancestor_record_offset":8828,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4589,"record_offset":9468,"parent_record_offset":9436,"ancestor_record_offset":8828,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4589,"record_offset":9664,"parent_record_offset":9628,"ancestor_record_offset":8828,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4589,"record_offset":9812,"parent_record_offset":9788,"ancestor_record_offset":8828,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4589,"record_offset":9836,"parent_record_offset":9812,"ancestor_record_offset":8828,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4589,"record_offset":10052,"parent_record_offset":10028,"ancestor_record_offset":8828,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4589,"record_offset":9884,"parent_record_offset":9860,"ancestor_record_offset":8828,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":9860,"parent_record_offset":9836,"ancestor_record_offset":8828,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4589,"record_offset":10028,"parent_record_offset":9628,"ancestor_record_offset":8828,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":9368,"parent_record_offset":9324,"ancestor_record_offset":8828,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":9324,"parent_record_offset":9288,"ancestor_record_offset":8828,"inlinee":5163,"source_line":445,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":9288,"parent_record_offset":9252,"ancestor_record_offset":8828,"inlinee":5162,"source_line":455,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":9404,"parent_record_offset":9324,"ancestor_record_offset":8828,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$6::visit_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":9436,"parent_record_offset":9404,"ancestor_record_offset":8828,"inlinee":6423,"source_line":603,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":9252,"parent_record_offset":9216,"ancestor_record_offset":8828,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$6::next_key_seed<core::marker::PhantomData<enum2$<serde::__private::de::content::Content> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4589,"record_offset":10164,"rva":40365136,"va":5409074256}
// 编译实例 RVAs: 0x267ec50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14267f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409074256,"raw_hex":"e8dd070000","size":5,"soft_fail":false,"target_va":5409076592,"va":5409074574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287d150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2089996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409074256,"raw_hex":"e80ce41f00","size":5,"soft_fail":false,"target_va":5411164496,"va":5409074495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-27988761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409074256,"raw_hex":"e8e7ec54fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5409074276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":10408,"parent_record_offset":10164,"ancestor_record_offset":10164,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":10744,"parent_record_offset":10164,"ancestor_record_offset":10164,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":10376,"parent_record_offset":10164,"ancestor_record_offset":10164,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":10468,"parent_record_offset":10164,"ancestor_record_offset":10164,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":4589,"record_offset":10440,"parent_record_offset":10164,"ancestor_record_offset":10164,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":10556,"parent_record_offset":10164,"ancestor_record_offset":10164,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4589,"record_offset":10596,"parent_record_offset":10556,"ancestor_record_offset":10164,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":10708,"parent_record_offset":10684,"ancestor_record_offset":10164,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$6::next_value_seed<core::marker::PhantomData<asset::texture::config::InheritableTextureOpts> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4589,"record_offset":10776,"rva":40365632,"va":5409074752}
// 编译实例 RVAs: 0x267ee40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1426823e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409074752,"raw_hex":"e80e350000","size":5,"soft_fail":false,"target_va":5409088480,"va":5409074893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28010962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409074752,"raw_hex":"e82e9654fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5409074973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28028110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409074752,"raw_hex":"e8325354fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5409074809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28003622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409074752,"raw_hex":"e8dab254fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5409074785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":11040,"parent_record_offset":10776,"ancestor_record_offset":10776,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":11188,"parent_record_offset":10776,"ancestor_record_offset":10776,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":10984,"parent_record_offset":10776,"ancestor_record_offset":10776,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":11280,"parent_record_offset":10776,"ancestor_record_offset":10776,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":11072,"parent_record_offset":10776,"ancestor_record_offset":10776,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":10776,"inlinee":125013,"source_line":17,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":11116,"parent_record_offset":11096,"ancestor_record_offset":10776,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$6::next_value_seed<core::marker::PhantomData<enum2$<serde::__private::de::content::Content> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4589,"record_offset":11340,"rva":40366128,"va":5409075248}
// 编译实例 RVAs: 0x267f030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14267f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409075248,"raw_hex":"e8af040000","size":5,"soft_fail":false,"target_va":5409076592,"va":5409075388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde::__private::de::content::Content> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409075248,"raw_hex":"e89bedffff","size":5,"soft_fail":false,"target_va":5409070992,"va":5409075696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28011461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409075248,"raw_hex":"e83b9454fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5409075472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28028621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409075248,"raw_hex":"e8335154fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5409075320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28004134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409075248,"raw_hex":"e8dab054fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5409075297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":11608,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":11788,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":11552,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":11880,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":11640,"parent_record_offset":11340,"ancestor_record_offset":11340,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":11712,"parent_record_offset":11688,"ancestor_record_offset":11340,"inlinee":38808,"source_line":32,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4589,"record_offset":11664,"parent_record_offset":11640,"ancestor_record_offset":11340,"inlinee":6541,"source_line":294,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Deserializer::__deserialize_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":11688,"parent_record_offset":11664,"ancestor_record_offset":11340,"inlinee":6543,"source_line":1223,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4589,"record_offset":5204,"rva":40362368,"va":5409071488}
// 编译实例 RVAs: 0x267e180, 0x267e380, 0x267e560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2409183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409071488,"raw_hex":"e8dfc22400","size":5,"soft_fail":false,"target_va":5411481152,"va":5409071964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::identifier
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28001870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409071488,"raw_hex":"e8b2b954fe","size":5,"soft_fail":false,"target_va":5381069696,"va":5409071561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28000353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409071488,"raw_hex":"e89fbf54fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5409071516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":5448,"parent_record_offset":5204,"ancestor_record_offset":5204,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":6196,"parent_record_offset":5204,"ancestor_record_offset":5204,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":5416,"parent_record_offset":5204,"ancestor_record_offset":5204,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":6168,"parent_record_offset":5204,"ancestor_record_offset":5204,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":5480,"parent_record_offset":5204,"ancestor_record_offset":5204,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4589,"record_offset":5676,"parent_record_offset":5640,"ancestor_record_offset":5204,"inlinee":108176,"source_line":4,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4589,"record_offset":5520,"parent_record_offset":5480,"ancestor_record_offset":5204,"inlinee":125478,"source_line":4,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4589,"record_offset":5744,"parent_record_offset":5712,"ancestor_record_offset":5204,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":5604,"parent_record_offset":5556,"ancestor_record_offset":5204,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":6092,"parent_record_offset":5556,"ancestor_record_offset":5204,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":5776,"parent_record_offset":5744,"ancestor_record_offset":5204,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":5808,"parent_record_offset":5776,"ancestor_record_offset":5204,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4589,"record_offset":5712,"parent_record_offset":5676,"ancestor_record_offset":5204,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":5556,"parent_record_offset":5520,"ancestor_record_offset":5204,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":5640,"parent_record_offset":5556,"ancestor_record_offset":5204,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<u32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4589,"record_offset":8084,"rva":40363840,"va":5409072960}
// 编译实例 RVAs: 0x267e740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14287c5e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2088464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409072960,"raw_hex":"e810de1f00","size":5,"soft_fail":false,"target_va":5411161568,"va":5409073099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::struct_error_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-27987629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409072960,"raw_hex":"e853f154fe","size":5,"soft_fail":false,"target_va":5381085680,"va":5409073304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28009422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409072960,"raw_hex":"e8329c54fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5409073433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28026322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409072960,"raw_hex":"e82e5a54fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5409073021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28001838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409072960,"raw_hex":"e8d2b954fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5409073001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":8416,"parent_record_offset":8084,"ancestor_record_offset":8084,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":8704,"parent_record_offset":8084,"ancestor_record_offset":8084,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":8648,"parent_record_offset":8084,"ancestor_record_offset":8084,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":8264,"parent_record_offset":8084,"ancestor_record_offset":8084,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":8768,"parent_record_offset":8084,"ancestor_record_offset":8084,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] ron::extensions::Extensions::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4589,"record_offset":8320,"parent_record_offset":8084,"ancestor_record_offset":8084,"inlinee":12958,"source_line":703,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":8448,"parent_record_offset":8084,"ancestor_record_offset":8084,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":8532,"parent_record_offset":8504,"ancestor_record_offset":8084,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_u32
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":8504,"parent_record_offset":8476,"ancestor_record_offset":8084,"inlinee":5581,"source_line":402,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$9::newtype_variant_seed::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":8672,"parent_record_offset":8648,"ancestor_record_offset":8084,"inlinee":13355,"source_line":932,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4589,"record_offset":8376,"parent_record_offset":8348,"ancestor_record_offset":8084,"inlinee":13353,"source_line":641,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4589,"record_offset":8348,"parent_record_offset":8320,"ancestor_record_offset":8084,"inlinee":12959,"source_line":703,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":8476,"parent_record_offset":8448,"ancestor_record_offset":8084,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::integer<u32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4746,"record_offset":16520,"rva":42452448,"va":5411161568}
// 编译实例 RVAs: 0x287c5e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$ron..parse..Integer$GT$::parse
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30080901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e87b0035fe","size":5,"soft_fail":false,"target_va":5381082560,"va":5411163456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$ron..parse..Integer$GT$::try_from_parsed_integer
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30080458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8360235fe","size":5,"soft_fail":false,"target_va":5381082592,"va":5411163045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<ron::parse::ParsedInteger>,enum2$<ron::error::Error> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e804000000","size":5,"soft_fail":false,"target_va":5411163552,"va":5411163543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t381410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8e2d10500","size":5,"soft_fail":false,"target_va":5411543360,"va":5411161945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30116146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8ce7634fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5411162845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_ident
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30096310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e84ac434fe","size":5,"soft_fail":false,"target_va":5381065728,"va":5411162033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30115125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8cb7a34fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5411161696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30116153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8c77634fe","size":5,"soft_fail":false,"target_va":5381045568,"va":5411161716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30114008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8287f34fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5411161891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_escape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30082977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e85ff834fe","size":5,"soft_fail":false,"target_va":5381078784,"va":5411161756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-8111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e851e0ffff","size":5,"soft_fail":false,"target_va":5411154160,"va":5411162266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-3699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e88df1ffff","size":5,"soft_fail":false,"target_va":5411158688,"va":5411162382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-4708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e89cedffff","size":5,"soft_fail":false,"target_va":5411157792,"va":5411162495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-5193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8b7ebffff","size":5,"soft_fail":false,"target_va":5411156896,"va":5411162084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-6838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e84ae5ffff","size":5,"soft_fail":false,"target_va":5411155968,"va":5411162801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-7940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8fce0ffff","size":5,"soft_fail":false,"target_va":5411155056,"va":5411162991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-9895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e859d9ffff","size":5,"soft_fail":false,"target_va":5411153232,"va":5411163122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-10296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8c8d7ffff","size":5,"soft_fail":false,"target_va":5411152320,"va":5411162611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::peek_char_or_eof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30115667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8ad7834fe","size":5,"soft_fail":false,"target_va":5381045968,"va":5411161630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30094103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8e9cc34fe","size":5,"soft_fail":false,"target_va":5381069184,"va":5411163282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_next_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30116201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8977634fe","size":5,"soft_fail":false,"target_va":5381045680,"va":5411161876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30115121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161568,"raw_hex":"e8cf7a34fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5411161612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4746,"record_offset":17612,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":18244,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":17408,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":16664,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":17700,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":16792,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":17352,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::set_cursor
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":17380,"parent_record_offset":16520,"ancestor_record_offset":16520,"inlinee":9150,"source_line":113,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4746,"record_offset":16920,"parent_record_offset":16888,"ancestor_record_offset":16520,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":17180,"parent_record_offset":17152,"ancestor_record_offset":16520,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":17152,"parent_record_offset":17100,"ancestor_record_offset":16520,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":16888,"parent_record_offset":16860,"ancestor_record_offset":16520,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":17072,"parent_record_offset":17044,"ancestor_record_offset":16520,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":17044,"parent_record_offset":17016,"ancestor_record_offset":16520,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::Pattern::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4746,"record_offset":16988,"parent_record_offset":16960,"ancestor_record_offset":16520,"inlinee":9155,"source_line":115,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$10::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4746,"record_offset":17016,"parent_record_offset":16988,"ancestor_record_offset":16520,"inlinee":8760,"source_line":689,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$17::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4746,"record_offset":16960,"parent_record_offset":16792,"ancestor_record_offset":16520,"inlinee":9153,"source_line":746,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":17852,"parent_record_offset":17816,"ancestor_record_offset":16520,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":17816,"parent_record_offset":17788,"ancestor_record_offset":16520,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":16860,"parent_record_offset":16828,"ancestor_record_offset":16520,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":16828,"parent_record_offset":16792,"ancestor_record_offset":16520,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":17100,"parent_record_offset":17072,"ancestor_record_offset":16520,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":17240,"parent_record_offset":17100,"ancestor_record_offset":16520,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":17212,"parent_record_offset":17100,"ancestor_record_offset":16520,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$3::integer::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":18276,"parent_record_offset":18244,"ancestor_record_offset":16520,"inlinee":9157,"source_line":427,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::integer<u8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4746,"record_offset":14700,"rva":42450464,"va":5411159584}
// 编译实例 RVAs: 0x287be20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u8$u20$as$u20$ron..parse..Integer$GT$::parse
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30079845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e89b0435fe","size":5,"soft_fail":false,"target_va":5381081632,"va":5411161472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u8$u20$as$u20$ron..parse..Integer$GT$::try_from_parsed_integer
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30079402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e8560635fe","size":5,"soft_fail":false,"target_va":5381081664,"va":5411161061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<ron::parse::ParsedInteger>,enum2$<ron::error::Error> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e8c4070000","size":5,"soft_fail":false,"target_va":5411163552,"va":5411161559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t383394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e8a2d90500","size":5,"soft_fail":false,"target_va":5411543360,"va":5411159961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30114162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e88e7e34fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5411160861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_ident
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30094326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e80acc34fe","size":5,"soft_fail":false,"target_va":5381065728,"va":5411160049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30113141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e88b8234fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5411159712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30114169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e8877e34fe","size":5,"soft_fail":false,"target_va":5381045568,"va":5411159732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30112024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e8e88634fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5411159907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_escape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30080993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e81f0035fe","size":5,"soft_fail":false,"target_va":5381078784,"va":5411159772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-6127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e811e8ffff","size":5,"soft_fail":false,"target_va":5411154160,"va":5411160282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-1715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e84df9ffff","size":5,"soft_fail":false,"target_va":5411158688,"va":5411160398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-2724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e85cf5ffff","size":5,"soft_fail":false,"target_va":5411157792,"va":5411160511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-3209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e877f3ffff","size":5,"soft_fail":false,"target_va":5411156896,"va":5411160100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-4854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e80aedffff","size":5,"soft_fail":false,"target_va":5411155968,"va":5411160817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-5956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e8bce8ffff","size":5,"soft_fail":false,"target_va":5411155056,"va":5411161007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-7911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e819e1ffff","size":5,"soft_fail":false,"target_va":5411153232,"va":5411161138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-8312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e888dfffff","size":5,"soft_fail":false,"target_va":5411152320,"va":5411160627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::peek_char_or_eof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30113683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e86d8034fe","size":5,"soft_fail":false,"target_va":5381045968,"va":5411159646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30092119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e8a9d434fe","size":5,"soft_fail":false,"target_va":5381069184,"va":5411161298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_next_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30114217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e8577e34fe","size":5,"soft_fail":false,"target_va":5381045680,"va":5411159892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30113137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411159584,"raw_hex":"e88f8234fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5411159628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4746,"record_offset":15792,"parent_record_offset":14700,"ancestor_record_offset":14700,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":16424,"parent_record_offset":14700,"ancestor_record_offset":14700,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":15588,"parent_record_offset":14700,"ancestor_record_offset":14700,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":14844,"parent_record_offset":14700,"ancestor_record_offset":14700,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":15880,"parent_record_offset":14700,"ancestor_record_offset":14700,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":14972,"parent_record_offset":14700,"ancestor_record_offset":14700,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":15532,"parent_record_offset":14700,"ancestor_record_offset":14700,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::set_cursor
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":15560,"parent_record_offset":14700,"ancestor_record_offset":14700,"inlinee":9150,"source_line":113,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4746,"record_offset":15100,"parent_record_offset":15068,"ancestor_record_offset":14700,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":15360,"parent_record_offset":15332,"ancestor_record_offset":14700,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":15332,"parent_record_offset":15280,"ancestor_record_offset":14700,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":15068,"parent_record_offset":15040,"ancestor_record_offset":14700,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":15252,"parent_record_offset":15224,"ancestor_record_offset":14700,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":15224,"parent_record_offset":15196,"ancestor_record_offset":14700,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::Pattern::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4746,"record_offset":15168,"parent_record_offset":15140,"ancestor_record_offset":14700,"inlinee":9155,"source_line":115,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$10::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4746,"record_offset":15196,"parent_record_offset":15168,"ancestor_record_offset":14700,"inlinee":8760,"source_line":689,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$17::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4746,"record_offset":15140,"parent_record_offset":14972,"ancestor_record_offset":14700,"inlinee":9153,"source_line":746,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":16032,"parent_record_offset":15996,"ancestor_record_offset":14700,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":15996,"parent_record_offset":15968,"ancestor_record_offset":14700,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":15040,"parent_record_offset":15008,"ancestor_record_offset":14700,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":15008,"parent_record_offset":14972,"ancestor_record_offset":14700,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":15280,"parent_record_offset":15252,"ancestor_record_offset":14700,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":15420,"parent_record_offset":15280,"ancestor_record_offset":14700,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":15392,"parent_record_offset":15280,"ancestor_record_offset":14700,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$3::integer::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":16456,"parent_record_offset":16424,"ancestor_record_offset":14700,"inlinee":9157,"source_line":427,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<i16>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4746,"record_offset":4024,"rva":42445040,"va":5411154160}
// 编译实例 RVAs: 0x287a8f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i16$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30073941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154160,"raw_hex":"e8ab1b35fe","size":5,"soft_fail":false,"target_va":5381080704,"va":5411154640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i16$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30073881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154160,"raw_hex":"e8e71b35fe","size":5,"soft_fail":false,"target_va":5381080688,"va":5411154564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i16$u20$as$u20$ron..parse..Num$GT$::checked_sub_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30074181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154160,"raw_hex":"e8bb1a35fe","size":5,"soft_fail":false,"target_va":5381080720,"va":5411154896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t388311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154160,"raw_hex":"e8d7ec0500","size":5,"soft_fail":false,"target_va":5411543360,"va":5411155044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30108199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154160,"raw_hex":"e8d99534fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5411154306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30107630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154160,"raw_hex":"e8129834fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5411154201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30106402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154160,"raw_hex":"e8de9c34fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5411154285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30107843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154160,"raw_hex":"e83d9734fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5411154334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":4152,"parent_record_offset":4024,"ancestor_record_offset":4024,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":4308,"parent_record_offset":4024,"ancestor_record_offset":4024,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4746,"record_offset":5076,"parent_record_offset":5044,"ancestor_record_offset":4024,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4746,"record_offset":4264,"parent_record_offset":4236,"ancestor_record_offset":4024,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4746,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":4024,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":4024,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":4776,"parent_record_offset":4748,"ancestor_record_offset":4024,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":4528,"parent_record_offset":4308,"ancestor_record_offset":4024,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4746,"record_offset":4408,"parent_record_offset":4384,"ancestor_record_offset":4024,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4746,"record_offset":4432,"parent_record_offset":4408,"ancestor_record_offset":4024,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":4024,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":4748,"parent_record_offset":4684,"ancestor_record_offset":4024,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":4024,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":4360,"parent_record_offset":4308,"ancestor_record_offset":4024,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":4384,"parent_record_offset":4360,"ancestor_record_offset":4024,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":4024,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":4656,"parent_record_offset":4620,"ancestor_record_offset":4024,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":4620,"parent_record_offset":4560,"ancestor_record_offset":4024,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":4212,"parent_record_offset":4180,"ancestor_record_offset":4024,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":4180,"parent_record_offset":4152,"ancestor_record_offset":4024,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":4684,"parent_record_offset":4656,"ancestor_record_offset":4024,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":4836,"parent_record_offset":4684,"ancestor_record_offset":4024,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":4808,"parent_record_offset":4684,"ancestor_record_offset":4024,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":4500,"parent_record_offset":4308,"ancestor_record_offset":4024,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":5044,"parent_record_offset":4560,"ancestor_record_offset":4024,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<i32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4746,"record_offset":12932,"rva":42449568,"va":5411158688}
// 编译实例 RVAs: 0x287baa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30078405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411158688,"raw_hex":"e83b0a35fe","size":5,"soft_fail":false,"target_va":5381080768,"va":5411159168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30078345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411158688,"raw_hex":"e8770a35fe","size":5,"soft_fail":false,"target_va":5381080752,"va":5411159092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$ron..parse..Num$GT$::checked_sub_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30078645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411158688,"raw_hex":"e84b0935fe","size":5,"soft_fail":false,"target_va":5381080784,"va":5411159424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t383785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411158688,"raw_hex":"e829db0500","size":5,"soft_fail":false,"target_va":5411543360,"va":5411159570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30112727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411158688,"raw_hex":"e8298434fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5411158834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30112158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411158688,"raw_hex":"e8628634fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5411158729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30110930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411158688,"raw_hex":"e82e8b34fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5411158813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30112371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411158688,"raw_hex":"e88d8534fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5411158862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":13060,"parent_record_offset":12932,"ancestor_record_offset":12932,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":13216,"parent_record_offset":12932,"ancestor_record_offset":12932,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4746,"record_offset":13984,"parent_record_offset":13952,"ancestor_record_offset":12932,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4746,"record_offset":13172,"parent_record_offset":13144,"ancestor_record_offset":12932,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4746,"record_offset":13364,"parent_record_offset":13340,"ancestor_record_offset":12932,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":13872,"parent_record_offset":13848,"ancestor_record_offset":12932,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":13684,"parent_record_offset":13656,"ancestor_record_offset":12932,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":13436,"parent_record_offset":13216,"ancestor_record_offset":12932,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4746,"record_offset":13316,"parent_record_offset":13292,"ancestor_record_offset":12932,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4746,"record_offset":13340,"parent_record_offset":13316,"ancestor_record_offset":12932,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":13848,"parent_record_offset":13824,"ancestor_record_offset":12932,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":13656,"parent_record_offset":13592,"ancestor_record_offset":12932,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":13824,"parent_record_offset":13800,"ancestor_record_offset":12932,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":13268,"parent_record_offset":13216,"ancestor_record_offset":12932,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":13292,"parent_record_offset":13268,"ancestor_record_offset":12932,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":13144,"parent_record_offset":13120,"ancestor_record_offset":12932,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":13564,"parent_record_offset":13528,"ancestor_record_offset":12932,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":13528,"parent_record_offset":13468,"ancestor_record_offset":12932,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":13120,"parent_record_offset":13088,"ancestor_record_offset":12932,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":13088,"parent_record_offset":13060,"ancestor_record_offset":12932,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":13592,"parent_record_offset":13564,"ancestor_record_offset":12932,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":13744,"parent_record_offset":13592,"ancestor_record_offset":12932,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":13716,"parent_record_offset":13592,"ancestor_record_offset":12932,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":13408,"parent_record_offset":13216,"ancestor_record_offset":12932,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":13952,"parent_record_offset":13468,"ancestor_record_offset":12932,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<i64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4746,"record_offset":11208,"rva":42448672,"va":5411157792}
// 编译实例 RVAs: 0x287b720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i64$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30077445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411157792,"raw_hex":"e8fb0d35fe","size":5,"soft_fail":false,"target_va":5381080832,"va":5411158272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i64$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30077385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411157792,"raw_hex":"e8370e35fe","size":5,"soft_fail":false,"target_va":5381080816,"va":5411158196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i64$u20$as$u20$ron..parse..Num$GT$::checked_sub_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30077685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411157792,"raw_hex":"e80b0d35fe","size":5,"soft_fail":false,"target_va":5381080848,"va":5411158528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t384679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411157792,"raw_hex":"e8a7de0500","size":5,"soft_fail":false,"target_va":5411543360,"va":5411158676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30111831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411157792,"raw_hex":"e8a98734fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5411157938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30111262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411157792,"raw_hex":"e8e28934fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5411157833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30110034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411157792,"raw_hex":"e8ae8e34fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5411157917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30111475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411157792,"raw_hex":"e80d8934fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5411157966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":11336,"parent_record_offset":11208,"ancestor_record_offset":11208,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":11492,"parent_record_offset":11208,"ancestor_record_offset":11208,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4746,"record_offset":12280,"parent_record_offset":12248,"ancestor_record_offset":11208,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4746,"record_offset":11448,"parent_record_offset":11420,"ancestor_record_offset":11208,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4746,"record_offset":11640,"parent_record_offset":11616,"ancestor_record_offset":11208,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":12168,"parent_record_offset":12144,"ancestor_record_offset":11208,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":11980,"parent_record_offset":11952,"ancestor_record_offset":11208,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":11740,"parent_record_offset":11492,"ancestor_record_offset":11208,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4746,"record_offset":11592,"parent_record_offset":11568,"ancestor_record_offset":11208,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4746,"record_offset":11616,"parent_record_offset":11592,"ancestor_record_offset":11208,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":11208,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":11952,"parent_record_offset":11888,"ancestor_record_offset":11208,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":12120,"parent_record_offset":12096,"ancestor_record_offset":11208,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":11544,"parent_record_offset":11492,"ancestor_record_offset":11208,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":11568,"parent_record_offset":11544,"ancestor_record_offset":11208,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":11208,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":11860,"parent_record_offset":11824,"ancestor_record_offset":11208,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":11824,"parent_record_offset":11772,"ancestor_record_offset":11208,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":11396,"parent_record_offset":11364,"ancestor_record_offset":11208,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":11364,"parent_record_offset":11336,"ancestor_record_offset":11208,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":11888,"parent_record_offset":11860,"ancestor_record_offset":11208,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":12040,"parent_record_offset":11888,"ancestor_record_offset":11208,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":12012,"parent_record_offset":11888,"ancestor_record_offset":11208,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":11684,"parent_record_offset":11492,"ancestor_record_offset":11208,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":12248,"parent_record_offset":11772,"ancestor_record_offset":11208,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<i8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4746,"record_offset":9440,"rva":42447776,"va":5411156896}
// 编译实例 RVAs: 0x287b3a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i8$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30076725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411156896,"raw_hex":"e8cb1035fe","size":5,"soft_fail":false,"target_va":5381080656,"va":5411157376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i8$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30076665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411156896,"raw_hex":"e8071135fe","size":5,"soft_fail":false,"target_va":5381080640,"va":5411157300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i8$u20$as$u20$ron..parse..Num$GT$::checked_sub_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30076965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411156896,"raw_hex":"e8db0f35fe","size":5,"soft_fail":false,"target_va":5381080672,"va":5411157632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t385576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411156896,"raw_hex":"e828e20500","size":5,"soft_fail":false,"target_va":5411543360,"va":5411157779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30110935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411156896,"raw_hex":"e8298b34fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5411157042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30110366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411156896,"raw_hex":"e8628d34fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5411156937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30109138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411156896,"raw_hex":"e82e9234fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5411157021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30110579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411156896,"raw_hex":"e88d8c34fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5411157070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":9568,"parent_record_offset":9440,"ancestor_record_offset":9440,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":9724,"parent_record_offset":9440,"ancestor_record_offset":9440,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4746,"record_offset":10492,"parent_record_offset":10460,"ancestor_record_offset":9440,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4746,"record_offset":9680,"parent_record_offset":9652,"ancestor_record_offset":9440,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4746,"record_offset":9872,"parent_record_offset":9848,"ancestor_record_offset":9440,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":10380,"parent_record_offset":10356,"ancestor_record_offset":9440,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":10192,"parent_record_offset":10164,"ancestor_record_offset":9440,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":9944,"parent_record_offset":9724,"ancestor_record_offset":9440,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4746,"record_offset":9824,"parent_record_offset":9800,"ancestor_record_offset":9440,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4746,"record_offset":9848,"parent_record_offset":9824,"ancestor_record_offset":9440,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":9440,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":10164,"parent_record_offset":10100,"ancestor_record_offset":9440,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":10332,"parent_record_offset":10308,"ancestor_record_offset":9440,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":9776,"parent_record_offset":9724,"ancestor_record_offset":9440,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":9800,"parent_record_offset":9776,"ancestor_record_offset":9440,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":9652,"parent_record_offset":9628,"ancestor_record_offset":9440,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":10072,"parent_record_offset":10036,"ancestor_record_offset":9440,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":10036,"parent_record_offset":9976,"ancestor_record_offset":9440,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":9628,"parent_record_offset":9596,"ancestor_record_offset":9440,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":9596,"parent_record_offset":9568,"ancestor_record_offset":9440,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":10100,"parent_record_offset":10072,"ancestor_record_offset":9440,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":10252,"parent_record_offset":10100,"ancestor_record_offset":9440,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":10224,"parent_record_offset":10100,"ancestor_record_offset":9440,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":9916,"parent_record_offset":9724,"ancestor_record_offset":9440,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":10460,"parent_record_offset":9976,"ancestor_record_offset":9440,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<u16>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4746,"record_offset":7616,"rva":42446848,"va":5411155968}
// 编译实例 RVAs: 0x287b000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u16$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30075477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155968,"raw_hex":"e8ab1535fe","size":5,"soft_fail":false,"target_va":5381080976,"va":5411156448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u16$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30075433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155968,"raw_hex":"e8d71535fe","size":5,"soft_fail":false,"target_va":5381080944,"va":5411156372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t386478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155968,"raw_hex":"e8aee50500","size":5,"soft_fail":false,"target_va":5411543360,"va":5411156877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30110007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155968,"raw_hex":"e8c98e34fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5411156114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30109438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155968,"raw_hex":"e8029134fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5411156009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30108210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155968,"raw_hex":"e8ce9534fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5411156093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30109651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155968,"raw_hex":"e82d9034fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5411156142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":7744,"parent_record_offset":7616,"ancestor_record_offset":7616,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":7900,"parent_record_offset":7616,"ancestor_record_offset":7616,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4746,"record_offset":8668,"parent_record_offset":8636,"ancestor_record_offset":7616,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4746,"record_offset":7856,"parent_record_offset":7828,"ancestor_record_offset":7616,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4746,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":7616,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":7616,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":8368,"parent_record_offset":8340,"ancestor_record_offset":7616,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":8120,"parent_record_offset":7900,"ancestor_record_offset":7616,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4746,"record_offset":8000,"parent_record_offset":7976,"ancestor_record_offset":7616,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4746,"record_offset":8024,"parent_record_offset":8000,"ancestor_record_offset":7616,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":7616,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":8340,"parent_record_offset":8276,"ancestor_record_offset":7616,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":8508,"parent_record_offset":8484,"ancestor_record_offset":7616,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":7952,"parent_record_offset":7900,"ancestor_record_offset":7616,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":7976,"parent_record_offset":7952,"ancestor_record_offset":7616,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":7616,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":8248,"parent_record_offset":8212,"ancestor_record_offset":7616,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":8212,"parent_record_offset":8152,"ancestor_record_offset":7616,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":7804,"parent_record_offset":7772,"ancestor_record_offset":7616,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":7772,"parent_record_offset":7744,"ancestor_record_offset":7616,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":8276,"parent_record_offset":8248,"ancestor_record_offset":7616,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":8428,"parent_record_offset":8276,"ancestor_record_offset":7616,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":8400,"parent_record_offset":8276,"ancestor_record_offset":7616,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":8092,"parent_record_offset":7900,"ancestor_record_offset":7616,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":8636,"parent_record_offset":8152,"ancestor_record_offset":7616,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$15::checked_sub_ext
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":9312,"parent_record_offset":8760,"ancestor_record_offset":7616,"inlinee":9135,"source_line":1365,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$15::from_u8
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":9336,"parent_record_offset":9312,"ancestor_record_offset":7616,"inlinee":9136,"source_line":1341,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<u32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4746,"record_offset":5792,"rva":42445936,"va":5411155056}
// 编译实例 RVAs: 0x287ac70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30074485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155056,"raw_hex":"e88b1935fe","size":5,"soft_fail":false,"target_va":5381081056,"va":5411155536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30074425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155056,"raw_hex":"e8c71935fe","size":5,"soft_fail":false,"target_va":5381081040,"va":5411155460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t387393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155056,"raw_hex":"e841e90500","size":5,"soft_fail":false,"target_va":5411543360,"va":5411155962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30109095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155056,"raw_hex":"e8599234fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5411155202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30108526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155056,"raw_hex":"e8929434fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5411155097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30107298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155056,"raw_hex":"e85e9934fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5411155181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30108739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411155056,"raw_hex":"e8bd9334fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5411155230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":5920,"parent_record_offset":5792,"ancestor_record_offset":5792,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":6076,"parent_record_offset":5792,"ancestor_record_offset":5792,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4746,"record_offset":6844,"parent_record_offset":6812,"ancestor_record_offset":5792,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4746,"record_offset":6032,"parent_record_offset":6004,"ancestor_record_offset":5792,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4746,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":5792,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":6732,"parent_record_offset":6708,"ancestor_record_offset":5792,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":6544,"parent_record_offset":6516,"ancestor_record_offset":5792,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":6296,"parent_record_offset":6076,"ancestor_record_offset":5792,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4746,"record_offset":6176,"parent_record_offset":6152,"ancestor_record_offset":5792,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4746,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":5792,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":6708,"parent_record_offset":6684,"ancestor_record_offset":5792,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":6516,"parent_record_offset":6452,"ancestor_record_offset":5792,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":5792,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":6128,"parent_record_offset":6076,"ancestor_record_offset":5792,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":6152,"parent_record_offset":6128,"ancestor_record_offset":5792,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":5792,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":6424,"parent_record_offset":6388,"ancestor_record_offset":5792,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":6388,"parent_record_offset":6328,"ancestor_record_offset":5792,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":5980,"parent_record_offset":5948,"ancestor_record_offset":5792,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":5948,"parent_record_offset":5920,"ancestor_record_offset":5792,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":6452,"parent_record_offset":6424,"ancestor_record_offset":5792,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":6604,"parent_record_offset":6452,"ancestor_record_offset":5792,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":6576,"parent_record_offset":6452,"ancestor_record_offset":5792,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":6268,"parent_record_offset":6076,"ancestor_record_offset":5792,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":6812,"parent_record_offset":6328,"ancestor_record_offset":5792,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$16::checked_sub_ext
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":7488,"parent_record_offset":6936,"ancestor_record_offset":5792,"inlinee":9132,"source_line":1365,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$16::from_u8
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":7512,"parent_record_offset":7488,"ancestor_record_offset":5792,"inlinee":9133,"source_line":1341,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4746,"record_offset":2204,"rva":42444112,"va":5411153232}
// 编译实例 RVAs: 0x287a550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u64$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30072581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411153232,"raw_hex":"e8fb2035fe","size":5,"soft_fail":false,"target_va":5381081136,"va":5411153712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u64$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30072537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411153232,"raw_hex":"e8272135fe","size":5,"soft_fail":false,"target_va":5381081104,"va":5411153636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t389214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411153232,"raw_hex":"e85ef00500","size":5,"soft_fail":false,"target_va":5411543360,"va":5411154141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30107271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411153232,"raw_hex":"e8799934fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5411153378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30106702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411153232,"raw_hex":"e8b29b34fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5411153273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30105474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411153232,"raw_hex":"e87ea034fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5411153357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30106915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411153232,"raw_hex":"e8dd9a34fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5411153406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":2332,"parent_record_offset":2204,"ancestor_record_offset":2204,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":2488,"parent_record_offset":2204,"ancestor_record_offset":2204,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4746,"record_offset":3256,"parent_record_offset":3224,"ancestor_record_offset":2204,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4746,"record_offset":2444,"parent_record_offset":2416,"ancestor_record_offset":2204,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4746,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":2204,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":2204,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":2204,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":2708,"parent_record_offset":2488,"ancestor_record_offset":2204,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4746,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":2204,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4746,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":2204,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":2204,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":2928,"parent_record_offset":2864,"ancestor_record_offset":2204,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":2204,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":2540,"parent_record_offset":2488,"ancestor_record_offset":2204,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":2204,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":2204,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":2836,"parent_record_offset":2800,"ancestor_record_offset":2204,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":2800,"parent_record_offset":2740,"ancestor_record_offset":2204,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":2392,"parent_record_offset":2360,"ancestor_record_offset":2204,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":2360,"parent_record_offset":2332,"ancestor_record_offset":2204,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":2864,"parent_record_offset":2836,"ancestor_record_offset":2204,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":3016,"parent_record_offset":2864,"ancestor_record_offset":2204,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":2988,"parent_record_offset":2864,"ancestor_record_offset":2204,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":2680,"parent_record_offset":2488,"ancestor_record_offset":2204,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":3224,"parent_record_offset":2740,"ancestor_record_offset":2204,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$17::checked_sub_ext
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":3896,"parent_record_offset":3348,"ancestor_record_offset":2204,"inlinee":9138,"source_line":1365,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$17::from_u8
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":2204,"inlinee":9139,"source_line":1341,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<u8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4746,"record_offset":388,"rva":42443200,"va":5411152320}
// 编译实例 RVAs: 0x287a1c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u8$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-30071909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411152320,"raw_hex":"e89b2335fe","size":5,"soft_fail":false,"target_va":5381080896,"va":5411152800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u8$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30071849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411152320,"raw_hex":"e8d72335fe","size":5,"soft_fail":false,"target_va":5381080880,"va":5411152724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t390131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411152320,"raw_hex":"e8f3f30500","size":5,"soft_fail":false,"target_va":5411543360,"va":5411153224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30106359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411152320,"raw_hex":"e8099d34fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5411152466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30105790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411152320,"raw_hex":"e8429f34fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5411152361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30104562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411152320,"raw_hex":"e80ea434fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5411152445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-30106003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411152320,"raw_hex":"e86d9e34fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5411152494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":516,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":672,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4746,"record_offset":1440,"parent_record_offset":1408,"ancestor_record_offset":388,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4746,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":388,"inlinee":9142,"source_line":653,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4746,"record_offset":628,"parent_record_offset":600,"ancestor_record_offset":388,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4746,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":388,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":388,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":1140,"parent_record_offset":1112,"ancestor_record_offset":388,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4746,"record_offset":892,"parent_record_offset":672,"ancestor_record_offset":388,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4746,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":388,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4746,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":388,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":388,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":1112,"parent_record_offset":1048,"ancestor_record_offset":388,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":388,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":724,"parent_record_offset":672,"ancestor_record_offset":388,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":748,"parent_record_offset":724,"ancestor_record_offset":388,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4746,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":388,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":1020,"parent_record_offset":984,"ancestor_record_offset":388,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4746,"record_offset":984,"parent_record_offset":924,"ancestor_record_offset":388,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":576,"parent_record_offset":544,"ancestor_record_offset":388,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4746,"record_offset":544,"parent_record_offset":516,"ancestor_record_offset":388,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":388,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":1200,"parent_record_offset":1048,"ancestor_record_offset":388,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4746,"record_offset":1172,"parent_record_offset":1048,"ancestor_record_offset":388,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":864,"parent_record_offset":672,"ancestor_record_offset":388,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":1408,"parent_record_offset":924,"ancestor_record_offset":388,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$14::checked_sub_ext
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4746,"record_offset":2084,"parent_record_offset":1532,"ancestor_record_offset":388,"inlinee":9141,"source_line":1365,"archive":"_pdb/B/modules/04746.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde::__private::de::content::impl$21::deserialize_enum<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>
// 记录形式: compiled_procedure; instances=4
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4605,"record_offset":5000,"rva":40443088,"va":5409152208}
// 编译实例 RVAs: 0x2691cd0, 0x2691dc0, 0x2691f10, 0x2691fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2327944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409152208,"raw_hex":"e888852300","size":5,"soft_fail":false,"target_va":5411480384,"va":5409152435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_value
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2328016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409152208,"raw_hex":"e8d0852300","size":5,"soft_fail":false,"target_va":5411480416,"va":5409152395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14268ca70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409152784,"raw_hex":"e828abffff","size":5,"soft_fail":false,"target_va":5409131120,"va":5409152835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1426914c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409152928,"raw_hex":"e8e2f4ffff","size":5,"soft_fail":false,"target_va":5409150144,"va":5409152985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1426917f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409152448,"raw_hex":"e8f2f9ffff","size":5,"soft_fail":false,"target_va":5409150960,"va":5409152505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142691970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409152208,"raw_hex":"e862fcffff","size":5,"soft_fail":false,"target_va":5409151344,"va":5409152265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142694b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409152208,"raw_hex":"e83c2e0000","size":5,"soft_fail":false,"target_va":5409164128,"va":5409152287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142922c60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2690455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409152448,"raw_hex":"e8970d2900","size":5,"soft_fail":false,"target_va":5411843168,"va":5409152708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142922ca0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2690817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409152208,"raw_hex":"e8010f2900","size":5,"soft_fail":false,"target_va":5411843232,"va":5409152410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4605,"record_offset":5232,"parent_record_offset":5000,"ancestor_record_offset":5000,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":5440,"parent_record_offset":5000,"ancestor_record_offset":5000,"inlinee":134115,"source_line":169,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4605,"record_offset":5408,"parent_record_offset":5000,"ancestor_record_offset":5000,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":5312,"parent_record_offset":5288,"ancestor_record_offset":5000,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":5360,"parent_record_offset":5336,"ancestor_record_offset":5000,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":5000,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4605,"record_offset":5288,"parent_record_offset":5260,"ancestor_record_offset":5000,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4605,"record_offset":5260,"parent_record_offset":5232,"ancestor_record_offset":5000,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":5492,"parent_record_offset":5440,"ancestor_record_offset":5000,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":5524,"parent_record_offset":5440,"ancestor_record_offset":5000,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_unit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4605,"record_offset":6148,"parent_record_offset":6120,"ancestor_record_offset":5560,"inlinee":71681,"source_line":1905,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$26::unit_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4605,"record_offset":6088,"parent_record_offset":6000,"ancestor_record_offset":5560,"inlinee":71680,"source_line":2135,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":5000,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":6120,"parent_record_offset":6088,"ancestor_record_offset":5560,"inlinee":6595,"source_line":33,"archive":"_pdb/B/modules/04605.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > (*)(ref$<enum2$<serde::__private::de::content::Content> >)
// 候选参数类型: ["ref$<enum2$<serde::__private::de::content::Content> >"]

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree_walker
// 信息 ID: 01e23c48c53bf1fb2207af93
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::texture::scan::scan_asset_tree @ 0x20d8c5a -> 0x20d8d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::texture::scan::scan_asset_tree_walker @ 0x20d96af -> 0x20d8d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::texture::scan::scan_asset_tree_walker @ 0x20d9878 -> 0x20d8d00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::Deserializer::handle_any_struct<serde::__private::de::content::ContentVisitor>
// 信息 ID: 4682a9f9da68b23a7e8af3d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::Deserializer::handle_any_struct -> ["serde::__private::de::content::ContentVisitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_any<serde::__private::de::content::ContentVisitor> @ 0x267fa77 -> 0x267d980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_any<serde::__private::de::content::ContentVisitor>
// 信息 ID: 7fe65093cb91a95f029981f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_any -> ["serde::__private::de::content::ContentVisitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::Deserializer::handle_any_struct<serde::__private::de::content::ContentVisitor> @ 0x267db03 -> 0x267f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$6::next_key_seed<core::marker::PhantomData<enum2$<serde::__private::de::content::Content> > > @ 0x267ed8e -> 0x267f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$6::next_value_seed<core::marker::PhantomData<enum2$<serde::__private::de::content::Content> > > @ 0x267f0bc -> 0x267f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$5::next_element_seed<core::marker::PhantomData<enum2$<serde::__private::de::content::Content> > > @ 0x267f3e5 -> 0x267f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> > @ 0x268aa8b -> 0x267f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,array$<u32,2> > @ 0x268af0d -> 0x267f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig> @ 0x268b28d -> 0x267f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabRtConfig> @ 0x268b79b -> 0x267f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$4::visit_some<ref_mut$<ron::de::Deserializer> > @ 0x2695380 -> 0x267f570 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$4::visit_map<ron::de::SerdeEnumContent> @ 0x2695545 -> 0x267f570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_byte_buf<serde::__private::de::content::ContentVisitor>
// 信息 ID: 72333af4dd9083a6d4309878
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_byte_buf -> ["serde::__private::de::content::ContentVisitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>
// 信息 ID: 1c84901dfd61f061e06fb266
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_struct -> ["asset::prefab::config::_::impl$0::deserialize::__Visitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::options::Options::from_str_seed<core::marker::PhantomData<asset::prefab::config::PrefabConfig>,asset::prefab::config::PrefabConfig> @ 0x242634e -> 0x26846c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>
// 信息 ID: e18a3e908f0915d8a529b9d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_struct -> ["asset::texture::config::_::impl$0::deserialize::__Visitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$6::next_value_seed<core::marker::PhantomData<asset::texture::config::InheritableTextureOpts> > @ 0x267eecd -> 0x26823e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$5::next_element_seed<core::marker::PhantomData<u8> >
// 信息 ID: e54237ff034fab4c5f3ab273
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["u8"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$5::next_element_seed -> ["core::marker::PhantomData<u8>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$19::deserialize::impl$0::visit_seq<u8,ron::de::CommaSeparated> @ 0x287a095 -> 0x267f240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$6::next_key_seed<core::marker::PhantomData<alloc::string::String> >
// 信息 ID: 64907abaf6dc70ebbe511bf5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$6::next_key_seed -> ["core::marker::PhantomData<alloc::string::String>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> > @ 0x267fec5 -> 0x267e9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState> > @ 0x2681116 -> 0x267e9e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$6::next_key_seed<core::marker::PhantomData<enum2$<serde::__private::de::content::Content> > >
// 信息 ID: ae3e103b243c76a8b847f931
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde::__private::de::content::Content"]; 字段布局 = unknown。
// [类型关系] core::marker::PhantomData -> ["enum2$<serde::__private::de::content::Content>"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$6::next_key_seed -> ["core::marker::PhantomData<enum2$<serde::__private::de::content::Content> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::__private::de::content::impl$4::visit_map<ron::de::CommaSeparated> @ 0x2695942 -> 0x267ec50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$6::next_value_seed<core::marker::PhantomData<asset::texture::config::InheritableTextureOpts> >
// 信息 ID: d21bb9e0f49470e90ca29feb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["asset::texture::config::InheritableTextureOpts"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$6::next_value_seed -> ["core::marker::PhantomData<asset::texture::config::InheritableTextureOpts>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState> > @ 0x2681155 -> 0x267ee40 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor> @ 0x26838cf -> 0x267ee40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$6::next_value_seed<core::marker::PhantomData<enum2$<serde::__private::de::content::Content> > >
// 信息 ID: b39719164299a3503b54b4e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde::__private::de::content::Content"]; 字段布局 = unknown。
// [类型关系] core::marker::PhantomData -> ["enum2$<serde::__private::de::content::Content>"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$6::next_value_seed -> ["core::marker::PhantomData<enum2$<serde::__private::de::content::Content> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::__private::de::content::impl$4::visit_map<ron::de::CommaSeparated> @ 0x2695981 -> 0x267f030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >
// 信息 ID: 6cc44f945f09b1afaf6a07bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["asset::texture::desc::_::impl$0::deserialize::__Field"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$8::variant_seed -> ["core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor> @ 0x2681b13 -> 0x267e180 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor> @ 0x2681f93 -> 0x267e380 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor> @ 0x2681d53 -> 0x267e560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<u32> >
// 信息 ID: d4f42c76a005dcd18cffd2cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["u32"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$9::newtype_variant_seed -> ["core::marker::PhantomData<u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor> @ 0x268212d -> 0x267e740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::integer<u32>
// 信息 ID: 2c75fcd2338f0d488278e0b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::integer -> ["u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<u32> > @ 0x267e7cb -> 0x287c5e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::integer<u8>
// 信息 ID: 0782b66770f266a0d21403d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::integer -> ["u8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$5::next_element_seed<core::marker::PhantomData<u8> > @ 0x267f29a -> 0x287be20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<i16>
// 信息 ID: 639889e6a7229028496c5682
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["i16"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x287c0da -> 0x287a8f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x287c89a -> 0x287a8f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<i32>
// 信息 ID: 46cda47c34f8fdc036cc47f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["i32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x287c14e -> 0x287baa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x287c90e -> 0x287baa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<i64>
// 信息 ID: 3bc9e15b7bb00d9b054d3d3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["i64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x287c1bf -> 0x287b720 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x287c97f -> 0x287b720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<i8>
// 信息 ID: 2b263b6eaeac4f90ffe80fb0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["i8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x287c024 -> 0x287b3a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x287c7e4 -> 0x287b3a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<u16>
// 信息 ID: da1bb673121ba2fb1cf2d66f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["u16"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x287c2f1 -> 0x287b000 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x287cab1 -> 0x287b000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<u32>
// 信息 ID: 7adca1a0360a86e65ba7d55f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x287c3af -> 0x287ac70 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x287cb6f -> 0x287ac70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<u64>
// 信息 ID: b63c462ae8755ec57f384c42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x287c432 -> 0x287a550 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x287cbf2 -> 0x287a550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<u8>
// 信息 ID: 51c7d7f3cf76e39e232cc666
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["u8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x287c233 -> 0x287a1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x287c9f3 -> 0x287a1c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde::__private::de::content::impl$21::deserialize_enum<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>
// 信息 ID: 0a0d7dc915b861d936375545
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e55b91db0419e2d4eecc727d: serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > (*)(ref$<enum2$<serde::__private::de::content::Content> >)
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::__private::de::content::impl$21::deserialize_enum -> ["enum2$<ron::error::Error>", "asset::prefab::config::_::impl$0::deserialize::__Visitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> > @ 0x268ab16 -> 0x2691cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> > @ 0x268ac7c -> 0x2691cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> > @ 0x268abd1 -> 0x2691cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x2694231 -> 0x2691dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x26942ae -> 0x2691dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::value::impl$45::next_element_seed<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde::__private::de::content::Content> >,serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > (*)(ref$<enum2$<serde::__private::de::content::Content> >)>,serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> >,enum2$<ron::error::Error>,core::marker::PhantomData<asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::__DeserializeWith> > @ 0x287e2ee -> 0x2691f10 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x26931bd -> 0x2691fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x26933e8 -> 0x2691fa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
